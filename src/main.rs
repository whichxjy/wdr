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

use manager::Manager;

fn main() {
    env_logger::init();

    let manager = Manager::new();
    manager.run();
}
