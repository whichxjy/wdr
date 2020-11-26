use crossbeam::channel::{bounded, tick, unbounded, Sender};
use crossbeam::select;
use std::collections::{HashMap, HashSet};
use std::str;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use wdrlib::config::{ProcessConfig, WdrConfig};
use wdrlib::info::{ProcessInfo, State};
use wdrlib::zk::ZkClient;
use wdrlib::{zk_node_path, ZK_CONFIG_PATH};

use crate::event::listen_event;
use crate::process::{self, Process};
use crate::setting::{get_wdr_node_name, ZK_CONNECT_STRING};

lazy_static! {
    pub static ref ZK_CLIENT: ZkClient =
        ZkClient::new(&ZK_CONNECT_STRING).expect("Fail to connect to zk");
    static ref WORKERS_LOCK: RwLock<HashMap<String, Worker>> = RwLock::new(HashMap::new());
}

pub struct Worker {
    pub version: String,
    pub stop_sender: Sender<()>,
}

impl Worker {
    fn new<T: Into<String>>(version: T, stop_sender: Sender<()>) -> Self {
        Worker {
            version: version.into(),
            stop_sender,
        }
    }
}

pub fn run() {
    let node_path = zk_node_path!(get_wdr_node_name());

    // Ensure the node path exists.
    match ZK_CLIENT.ensure(&node_path) {
        Ok(()) => fn_info!("Wdr node path: {}", node_path),
        Err(err) => {
            fn_error!("Fail to create zk node path {}: {}", node_path, err);
            return;
        }
    };

    let mut prev_wdr_config = WdrConfig::default();
    let check_config_ticker = tick(Duration::new(5, 0));

    let (process_info_sender, process_info_receiver) = bounded(1);
    let (stop_done_sender, stop_done_receiver) = unbounded();
    let (quit_sender, quit_receiver) = bounded(1);

    ctrlc::set_handler(move || {
        quit_sender.send(()).unwrap();
    })
    .expect("Error setting Ctrl-C handler");

    // Listen event.
    thread::spawn(move || {
        listen_event(process_info_receiver);
    });

    loop {
        select! {
            // Check config every 5 seconds.
            recv(check_config_ticker) -> _ => {
                let wdr_config = match read_config() {
                    Some(wdr_config) => wdr_config,
                    None => {
                        fn_error!("Fail to read config:");
                        continue;
                    }
                };
                fn_debug!("Read config: {:?}", wdr_config);

                if wdr_config != prev_wdr_config {
                    fn_debug!("Config was updated");
                    flush_all_processes(wdr_config.configs.clone(), &process_info_sender, &stop_done_sender);
                    prev_wdr_config = wdr_config;
                }
            }
            // Quit.
            recv(quit_receiver) -> _ => {
                let workers = WORKERS_LOCK
                    .read()
                    .unwrap();

                let mut worker_count = 0;

                for (_, worker) in workers.iter() {
                    worker.stop_sender.send(()).unwrap();
                    worker_count += 1;
                }

                for _ in 0..worker_count {
                    stop_done_receiver.recv().unwrap();
                }

                break;
            }
        }
    }

    fn_info!("Quit wdr");
}

fn read_config() -> Option<WdrConfig> {
    // Read config.
    let raw_data = match ZK_CLIENT.get_data(&ZK_CONFIG_PATH) {
        Ok(config_data) => config_data,
        Err(err) => {
            fn_error!("Fail to get raw data from zk: {}", err);
            return None;
        }
    };

    let data = match str::from_utf8(&raw_data) {
        Ok(data) => data,
        Err(err) => {
            fn_error!("Fail to convert raw data: {}", err);
            return None;
        }
    };

    WdrConfig::from_str(data)
}

fn flush_all_processes(
    process_configs: Vec<ProcessConfig>,
    process_info_sender: &Sender<ProcessInfo>,
    stop_done_sender: &Sender<()>,
) {
    let mut valid_process_names: HashSet<String> = HashSet::new();

    for process_config in process_configs {
        valid_process_names.insert(process_config.name.to_owned());

        let process_info_sender = process_info_sender.to_owned();
        let stop_done_sender = stop_done_sender.to_owned();

        thread::spawn(move || {
            flush_process(process_config, process_info_sender, stop_done_sender);
        });
    }

    clear_useless_processes(valid_process_names);
}

fn flush_process(
    process_config: ProcessConfig,
    process_info_sender: Sender<ProcessInfo>,
    stop_done_sender: Sender<()>,
) {
    {
        let mut workers = WORKERS_LOCK.write().unwrap();

        if let Some(old_worker) = workers.get_mut(process_config.name.as_str()) {
            if process_config.version == old_worker.version {
                return;
            }

            // Stop old process.
            workers
                .get_mut(&process_config.name)
                .unwrap()
                .stop_sender
                .send(())
                .unwrap();
        }
    }

    let (stop_sender, stop_receiver) = bounded(1);

    let mut new_process = Process {
        config: process_config.clone(),
        state_lock: RwLock::new(State::Init),
        process_info_sender,
        stop_receiver,
        stop_done_sender,
    };

    // TODO: Retry.
    if process::prepare(&mut new_process).is_none() {
        fn_error!("Fail to prepare process {}", process_config.name);
        return;
    }

    if process::run(new_process).is_none() {
        fn_error!("Fail to run process {}", process_config.name);
        return;
    }

    WORKERS_LOCK.write().unwrap().insert(
        process_config.name,
        Worker::new(process_config.version, stop_sender),
    );
}

fn clear_useless_processes(valid_process_names: HashSet<String>) {
    let mut useless_process_names: HashSet<String> = HashSet::new();

    {
        let workers = WORKERS_LOCK.read().unwrap();

        for name in workers.keys() {
            if !valid_process_names.contains(name.as_str()) {
                useless_process_names.insert(name.to_owned());
            }
        }
    }

    let mut workers = WORKERS_LOCK.write().unwrap();

    for useless_process_name in useless_process_names {
        workers
            .get_mut(&useless_process_name)
            .unwrap()
            .stop_sender
            .send(())
            .unwrap();

        workers.remove(&useless_process_name);
        fn_info!("Process {} is clear", useless_process_name);
    }
}
