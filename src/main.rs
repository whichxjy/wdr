extern crate zookeeper;
#[macro_use]
extern crate log;
#[macro_use]
extern crate stdext;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate custom_error;

#[macro_use]
mod logger;
mod config;
mod manager;
mod model;
mod process;
mod zk;

use crate::config::ZK_CONNECT_STRING;
use manager::Manager;
use zk::ZkClient;

fn main() {
    env_logger::init();

    let zk_client = match ZkClient::new(&ZK_CONNECT_STRING) {
        Ok(zk_client) => zk_client,
        Err(err) => {
            wdr_error!("Fail to connect to zk: {}", err);
            return;
        }
    };

    let mut manager = Manager::new(zk_client);
    manager.run();
}
