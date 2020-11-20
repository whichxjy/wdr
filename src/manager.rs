use crate::model::WdrConfig;
use crate::zk::ZkClient;
use std::str;
use zookeeper::{CreateMode, ZkError};

pub fn run() {
    if let Err(err) = write_config() {
        wdr_error!("Fail to write config: {}", err);
        return;
    }

    if let Some(wdr_config) = read_config() {
        wdr_debug!("Read config: {:?}", wdr_config);
    }
}

fn write_config() -> Result<(), ZkError> {
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
        // Create a new node and write config.
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

fn read_config() -> Option<WdrConfig> {
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
        // Create a new node and write config.
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
