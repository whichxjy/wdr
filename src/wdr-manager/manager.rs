use wdrlib::zk::ZkClient;
use wdrlib::ZK_CONFIG_PATH;
use zookeeper::{CreateMode, ZkError};

use crate::setting::ZK_CONNECT_STRING;

pub fn write_config() -> Result<(), ZkError> {
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
                "resource": "https://whichxjy.com/hello",
                "cmd": "./hello"
            },
            {
                "name": "what",
                "version": "1",
                "resource": "https://whichxjy.com/what",
                "cmd": "./what"
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
