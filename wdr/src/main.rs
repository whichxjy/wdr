#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate fnlog;

mod event;
mod manager;
mod process;
mod settings;

fn main() {
    env_logger::init();
    settings::init();
    manager::run();
}
