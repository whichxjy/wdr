use crossbeam::channel::tick;
use std::collections::HashMap;
use std::str;
use std::time::Duration;
use zookeeper::CreateMode;

use crate::config::ZK_CONFIG_PATH;
use crate::model::{ProcessConfig, WdrConfig};
use crate::process::Process;
use crate::zk::ZkClient;

pub struct Manager {
    zk_client: ZkClient,
    prev_wdr_config: WdrConfig,
    processes: HashMap<String, Process>,
}

impl Manager {
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

        loop {
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
            self.flush_process(process_config);
        }
    }

    fn flush_process(&mut self, process_config: &ProcessConfig) {
        if let Some(old_process) = self.processes.get_mut(process_config.name.as_str()) {
            if process_config.version == old_process.config.version {
                return;
            }

            // Stop old process.
            old_process.stop();
        }

        let mut new_process = Process::new(process_config.to_owned());

        if let Err(err) = new_process.prepare() {
            wdr_error!("{}", err);
            return;
        }

        if let Err(err) = new_process.run() {
            wdr_error!("{}", err);
            return;
        }

        self.processes
            .insert(process_config.name.to_owned(), new_process);
    }
}
