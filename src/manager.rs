use crate::config::WORKSPACE_PATH;
use crate::model::WdrConfig;
use crate::zk::ZkClient;
use reqwest::blocking::Client as HttpClient;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::process::Command;
use std::str;
use url::Url;
use zookeeper::{CreateMode, ZkError};

pub struct Manager {
    http_client: HttpClient,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            http_client: HttpClient::new(),
        }
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
        let connect_string = "localhost:2181";

        let zk_client = match ZkClient::new(connect_string) {
            Ok(zk_client) => zk_client,
            Err(err) => return Err(err),
        };

        let path = "/config";

        let data = r#"
        {
            "configs": [
                {
                    "name": "hello",
                    "version": "1",
                    "resource": "https://whichxjy.com/hello"
                }
           ]
        }"#;

        if !zk_client.exists(path) {
            // Create a new node.
            if let Err(err) = zk_client.create(path, CreateMode::Persistent) {
                return Err(err);
            }
        }

        // Write config.
        if let Err(err) = zk_client.set_data(path, data.as_bytes().to_vec()) {
            return Err(err);
        }

        Ok(())
    }

    fn read_config(&self) -> Option<WdrConfig> {
        let connect_string = "localhost:2181";

        let zk_client = match ZkClient::new(connect_string) {
            Ok(zk_client) => zk_client,
            Err(err) => {
                wdr_error!("{}", err);
                return None;
            }
        };

        let path = "/config";

        if !zk_client.exists(path) {
            // Create a new node.
            if let Err(err) = zk_client.create(path, CreateMode::Persistent) {
                wdr_error!("{}", err);
                return None;
            }
        }

        // Read config.
        match zk_client.get_data(path) {
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
            self.download(&process_config.resource);
        }
    }

    fn download(&self, resource: &str) {
        wdr_info!("Start download from {}", resource);

        let url = match Url::parse(resource) {
            Ok(url) => url,
            Err(err) => {
                wdr_error!("Invalid URL: {}", err);
                return;
            }
        };

        let segments = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
        let filename = match segments.last() {
            Some(filename) => filename,
            None => {
                wdr_error!("Fail to parse filename from {}", resource);
                return;
            }
        };

        let full_path = WORKSPACE_PATH.join(filename);
        wdr_info!("Full path: {}", full_path.to_str().unwrap());

        let res = match self.http_client.get(resource).send() {
            Ok(res) => res,
            Err(err) => {
                wdr_error!("Fail to download: {}", err);
                return;
            }
        };

        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o777)
            .open(&full_path)
        {
            Ok(file) => file,
            Err(err) => {
                wdr_error!("Fail to open file {}: {}", filename, err);
                return;
            }
        };

        let bytes = match res.bytes() {
            Ok(bytes) => bytes,
            Err(err) => {
                wdr_error!("Fail to read bytes from response: {}", err);
                return;
            }
        };

        if let Err(err) = file.write_all(&bytes) {
            wdr_error!("Fail to write bytes to file: {}", err);
        }

        let output = Command::new("sh")
            .args(&["-c", "echo what"])
            .output()
            .expect("failed to execute process");

        wdr_info!("output: {}", str::from_utf8(&output.stdout).unwrap());
    }
}
