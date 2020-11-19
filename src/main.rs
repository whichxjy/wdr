#[macro_use]
extern crate log;
#[macro_use]
extern crate stdext;
extern crate zookeeper;

#[macro_use]
mod logger;
mod model;
mod zk;

use model::WdrConfig;
use std::str;
use zk::ZkClient;
use zookeeper::CreateMode;

fn main() {
    env_logger::init();

    let connect_string = "localhost:2181";

    let zk_client = match ZkClient::new(connect_string) {
        Ok(zk_client) => {
            wdr_info!("Connected to zk: {}", connect_string);
            zk_client
        }
        Err(err) => {
            wdr_error!("{}", err);
            return;
        }
    };

    let path = "/config";

    let data = r#"
    {
        "configs": [
            {
                "name": "hello",
                "version": "1"
            }
       ]
    }"#;

    if !zk_client.exists(path) {
        // Create a new node and write config.
        if let Err(err) = zk_client.create(path, CreateMode::Persistent) {
            wdr_error!("{}", err);
            return;
        }
    }

    // Write config.
    if let Err(err) = zk_client.set_data(path, data.as_bytes().to_vec()) {
        wdr_error!("{}", err);
        return;
    }

    // Read config.
    match zk_client.get_data(path) {
        Ok(config_data) => {
            let config_data = str::from_utf8(&config_data).unwrap();
            let wdr_config: WdrConfig = serde_json::from_str(config_data).unwrap();
            println!("deserialized = {:?}", wdr_config);
        }
        Err(err) => println!("{:?}", err),
    }
}
