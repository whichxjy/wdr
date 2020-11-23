#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate crossbeam;
#[macro_use]
extern crate fnlog;

#[macro_use]
mod setting;
mod manager;
mod process;

fn main() {
    env_logger::init();
    manager::run();
}
