#[macro_use]
extern crate log;
#[macro_use]
extern crate stdext;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate crossbeam;

#[macro_use]
mod logger;
mod config;
mod manager;
mod model;
mod process;
mod zk;

fn main() {
    env_logger::init();
    manager::run();
}
