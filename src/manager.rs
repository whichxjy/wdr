use crate::model::WdrConfig;
use crate::zk::ZkClient;
use std::str;
use zookeeper::CreateMode;

pub fn run() {
    let connect_string = "localhost:2181";

    let zk_client = match ZkClient::new(connect_string) {
        Ok(zk_client) => zk_client,
        Err(err) => {
            wdr_error!("{}", err);
            return;
        }
    };
    wdr_info!("Connected to zk: {}", connect_string);

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
    let wdr_config = match zk_client.get_data(path) {
        Ok(config_data) => {
            let config_data = match str::from_utf8(&config_data) {
                Ok(config_data) => config_data,
                Err(err) => {
                    wdr_error!("{}", err);
                    return;
                }
            };

            match WdrConfig::from_str(config_data) {
                Ok(wdr_config) => wdr_config,
                _ => {
                    wdr_error!("Fail to build config");
                    return;
                }
            }
        }
        Err(err) => {
            wdr_error!("{}", err);
            return;
        }
    };

    wdr_debug!("Read config: {:?}", wdr_config);
}
