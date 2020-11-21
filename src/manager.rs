use crate::config::{ZK_CONFIG_PATH, ZK_CONNECT_STRING};
use crate::model::WdrConfig;
use crate::process::Process;
use crate::zk::ZkClient;
use std::str;
use zookeeper::{CreateMode, ZkError};

pub struct Manager {}

impl Manager {
    pub fn new() -> Self {
        Manager {}
    }

    pub fn run(&self) {
        if let Err(err) = self.write_config() {
            wdr_error!("Fail to write config: {}", err);
            return;
        }

        if let Some(wdr_config) = self.read_config() {
            wdr_debug!("Read config: {:?}", wdr_config);
            self.run_processes(wdr_config);
        }
    }

    fn write_config(&self) -> Result<(), ZkError> {
        let zk_client = match ZkClient::new(&ZK_CONNECT_STRING) {
            Ok(zk_client) => zk_client,
            Err(err) => return Err(err),
        };

        let data = r#"
        {
            "configs": [
                {
                    "name": "hello",
                    "version": "1",
                    "resource": {
                        "link": "https://whichxjy.com/hello"
                    }
                }
           ]
        }"#;

        if !zk_client.exists(&ZK_CONFIG_PATH) {
            // Create a new node.
            if let Err(err) = zk_client.create(&ZK_CONFIG_PATH, CreateMode::Persistent) {
                return Err(err);
            }
        }

        // Write config.
        if let Err(err) = zk_client.set_data(&ZK_CONFIG_PATH, data.as_bytes().to_vec()) {
            return Err(err);
        }

        Ok(())
    }

    fn read_config(&self) -> Option<WdrConfig> {
        let zk_client = match ZkClient::new(&ZK_CONNECT_STRING) {
            Ok(zk_client) => zk_client,
            Err(err) => {
                wdr_error!("{}", err);
                return None;
            }
        };

        if !zk_client.exists(&ZK_CONFIG_PATH) {
            // Create a new node.
            if let Err(err) = zk_client.create(&ZK_CONFIG_PATH, CreateMode::Persistent) {
                wdr_error!("{}", err);
                return None;
            }
        }

        // Read config.
        match zk_client.get_data(&ZK_CONFIG_PATH) {
            Ok(config_data) => {
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
            _ => None,
        }
    }

    fn run_processes(&self, wdr_config: WdrConfig) {
        for process_config in wdr_config.configs {
            let mut p = Process::new(&process_config.name, &process_config.resource);

            if let Err(err) = p.prepare() {
                wdr_error!("{}", err);
                continue;
            }

            if let Err(err) = p.run() {
                wdr_error!("{}", err);
            }

            p.kill();
        }
    }
}
