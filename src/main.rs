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

fn main() {
    env_logger::init();
    manager::run();
}
