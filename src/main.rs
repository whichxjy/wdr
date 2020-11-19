extern crate zookeeper;

use serde::{Deserialize, Serialize};
use std::str;
use std::time::Duration;
use zookeeper::{Acl, CreateMode, WatchedEvent, Watcher, ZooKeeper};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
struct WdrConfig {
    configs: Vec<ProcessConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
struct ProcessConfig {
    name: String,
    version: String,
}

struct LoggingWatcher;
impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
        println!("{:?}", e)
    }
}

fn main() {
    let zk_urls = "localhost:2181";

    let zk_client = ZooKeeper::connect(&*zk_urls, Duration::from_secs(15), LoggingWatcher).unwrap();

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

    if zk_client.exists(path, false).unwrap().is_none() {
        // Create a new node and write config.
        if let Err(err) = zk_client.create(
            path,
            data.as_bytes().to_vec(),
            Acl::open_unsafe().clone(),
            CreateMode::Persistent,
        ) {
            println!("{:?}", err);
        }
    }

    // Read config.
    match zk_client.get_data(path, false) {
        Ok((config_data, _)) => {
            let config_data = str::from_utf8(&config_data).unwrap();
            let wdr_config: WdrConfig = serde_json::from_str(config_data).unwrap();
            println!("deserialized = {:?}", wdr_config);
        }
        Err(err) => println!("{:?}", err),
    }
}
