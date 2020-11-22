use crossbeam::channel::{bounded, tick, Sender};
use std::collections::{HashMap, HashSet};
use std::str;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use zookeeper::CreateMode;

use crate::config::{ZK_CONFIG_PATH, ZK_CONNECT_STRING};
use crate::model::{ProcessConfig, WdrConfig};
use crate::process;
use crate::zk::ZkClient;

lazy_static! {
    static ref ZK_CLIENT: ZkClient =
        ZkClient::new(&ZK_CONNECT_STRING).expect("Fail to connect to zk");
    static ref CURR_WDR_CONFIG_LOCK: RwLock<WdrConfig> = RwLock::new(WdrConfig::default());
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
    // Check config every 5 seconds.
    let check_config_ticker = tick(Duration::new(5, 0));

    let (quit_sender, quit_receiver) = bounded(1);

    ctrlc::set_handler(move || {
        let _ = quit_sender.send(());
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        select! {
            // Check config.
            recv(check_config_ticker) -> _ => {
                let wdr_config = match read_config() {
                    Some(wdr_config) => wdr_config,
                    None => {
                        wdr_error!("Fail to read config:");
                        continue;
                    }
                };
                wdr_debug!("Read config: {:?}", wdr_config);

                if wdr_config != *CURR_WDR_CONFIG_LOCK.read().unwrap() {
                    {
                        let mut curr_wdr_config = CURR_WDR_CONFIG_LOCK.write().unwrap();
                        *curr_wdr_config = wdr_config;
                    }

                    let curr_wdr_config = CURR_WDR_CONFIG_LOCK.read().unwrap();
                    flush_all_processes(curr_wdr_config.configs.clone());
                }
            }
            // Quit.
            recv(quit_receiver) -> _ => {
                let workers = WORKERS_LOCK
                    .write()
                    .unwrap();

                for (_, worker) in workers.iter() {
                    worker.stop_sender.send(()).unwrap();
                }

                break;
            }
        }
    }

    wdr_info!("Quit wdr");
}

fn read_config() -> Option<WdrConfig> {
    if !ZK_CLIENT.exists(&ZK_CONFIG_PATH) {
        // Create a new node.
        if let Err(err) = ZK_CLIENT.create(&ZK_CONFIG_PATH, CreateMode::Persistent) {
            wdr_error!("{}", err);
            return None;
        }
    }

    // Read config.
    let config_data = match ZK_CLIENT.get_data(&ZK_CONFIG_PATH) {
        Ok(config_data) => config_data,
        _ => return None,
    };

    let config_data = match str::from_utf8(&config_data) {
        Ok(config_data) => config_data,
        Err(err) => {
            wdr_error!("{}", err);
            return None;
        }
    };

    match WdrConfig::from_str(config_data) {
        Some(wdr_config) => Some(wdr_config),
        None => None,
    }
}

fn flush_all_processes(process_configs: Vec<ProcessConfig>) {
    let mut valid_process_names: HashSet<String> = HashSet::new();

    for process_config in process_configs {
        valid_process_names.insert(process_config.name.to_owned());

        thread::spawn(move || {
            flush_process(process_config);
        });
    }

    clear_useless_processes(valid_process_names);
}

fn flush_process(process_config: ProcessConfig) {
    if let Some(old_worker) = WORKERS_LOCK
        .write()
        .unwrap()
        .get_mut(process_config.name.as_str())
    {
        if process_config.version == old_worker.version {
            return;
        }

        // Stop old process.
        let _ = WORKERS_LOCK
            .write()
            .unwrap()
            .get_mut(&process_config.name)
            .unwrap()
            .stop_sender
            .send(());
    }

    let (stop_sender, stop_receiver) = bounded(1);

    // TODO: Retry.
    if let Err(err) = process::prepare(&process_config) {
        wdr_error!("{}", err);
        return;
    }

    if let Err(err) = process::run(process_config.to_owned(), stop_receiver) {
        wdr_error!("{}", err);
        return;
    }

    WORKERS_LOCK.write().unwrap().insert(
        process_config.name,
        Worker::new(process_config.version, stop_sender),
    );
}

fn clear_useless_processes(valid_process_names: HashSet<String>) {
    let mut useless_process_names: HashSet<String> = HashSet::new();

    for name in WORKERS_LOCK.read().unwrap().keys() {
        if !valid_process_names.contains(name.as_str()) {
            useless_process_names.insert(name.to_owned());
        }
    }

    for useless_process_name in useless_process_names {
        let _ = WORKERS_LOCK
            .write()
            .unwrap()
            .get_mut(&useless_process_name)
            .unwrap()
            .stop_sender
            .send(());

        WORKERS_LOCK.write().unwrap().remove(&useless_process_name);
        wdr_info!("Process {} is clear", useless_process_name);
    }
}
