#[macro_use]
extern crate log;
#[macro_use]
extern crate stdext;
extern crate zookeeper;

#[macro_use]
mod logger;
mod manager;
mod model;
mod zk;

use manager::Manager;

fn main() {
    env_logger::init();

    let manager = Manager::new();
    manager.run();
}
