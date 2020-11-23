#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate crossbeam;
#[macro_use]
extern crate wdrlib;

#[macro_use]
mod config;
mod manager;
mod process;
mod zk;

fn main() {
    env_logger::init();
    manager::run();
}
