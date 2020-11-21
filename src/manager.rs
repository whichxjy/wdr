use crossbeam::channel::tick;
use std::collections::HashMap;
use std::str;
use std::time::Duration;
use zookeeper::CreateMode;

use crate::config::ZK_CONFIG_PATH;
use crate::model::{ProcessConfig, WdrConfig};
use crate::process::Process;
use crate::zk::ZkClient;

pub struct Manager<'a> {
    zk_client: ZkClient,
    prev_wdr_config: WdrConfig,
    processes: HashMap<&'a str, Process<'a>>,
}

impl<'a> Manager<'a> {
    pub fn new(zk_client: ZkClient) -> Self {
        Manager {
            zk_client,
            prev_wdr_config: WdrConfig::default(),
            processes: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        // Check config every 10 seconds.
        let ticker = tick(Duration::new(10, 0));

        for _ in 0..5 {
            ticker.recv().unwrap();

            let wdr_config = match self.read_config() {
                Some(wdr_config) => wdr_config,
                None => {
                    wdr_error!("Fail to read config:");
                    continue;
                }
            };
            wdr_debug!("Read config: {:?}", wdr_config);

            if wdr_config != self.prev_wdr_config {
                self.flush_all_processes(&wdr_config.configs);
                self.prev_wdr_config = wdr_config;
            }
        }
    }

    fn read_config(&self) -> Option<WdrConfig> {
        if !self.zk_client.exists(&ZK_CONFIG_PATH) {
            // Create a new node.
            if let Err(err) = self
                .zk_client
                .create(&ZK_CONFIG_PATH, CreateMode::Persistent)
            {
                wdr_error!("{}", err);
                return None;
            }
        }

        // Read config.
        let config_data = match self.zk_client.get_data(&ZK_CONFIG_PATH) {
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

    fn flush_all_processes(&mut self, process_configs: &[ProcessConfig]) {
        for process_config in process_configs {
            let mut need_stop_old_process = false;
            let mut old_process: Option<&mut Process> = None;

            if let Some(p) = self.processes.get_mut(process_config.name.as_str()) {
                if process_config.version == p.config.version {
                    return;
                }

                old_process = Some(p);
                need_stop_old_process = true;
            }

            let mut new_process = Process::new(process_config);

            if let Err(err) = new_process.prepare() {
                wdr_error!("{}", err);
                continue;
            }

            if let Err(err) = new_process.run() {
                wdr_error!("{}", err);
                continue;
            }

            if need_stop_old_process {
                old_process.unwrap().stop();
            }
        }
    }
}
