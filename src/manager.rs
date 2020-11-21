use crossbeam::channel::tick;
use std::str;
use std::time::Duration;
use zookeeper::CreateMode;

use crate::config::ZK_CONFIG_PATH;
use crate::model::WdrConfig;
use crate::process::Process;
use crate::zk::ZkClient;

pub struct Manager {
    zk_client: ZkClient,
    prev_wdr_config: WdrConfig,
}

impl Manager {
    pub fn new(zk_client: ZkClient) -> Self {
        Manager {
            zk_client,
            prev_wdr_config: WdrConfig::default(),
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
                self.flush_all_processes(&wdr_config);
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

    fn flush_all_processes(&self, wdr_config: &WdrConfig) {
        for process_config in &wdr_config.configs {
            let mut p = Process::new(process_config);

            if let Err(err) = p.prepare() {
                wdr_error!("{}", err);
                continue;
            }

            if let Err(err) = p.run() {
                wdr_error!("{}", err);
                continue;
            }
        }
    }
}
