#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate fnlog;

mod manager;
mod process;
mod setting;
mod info;

use setting::get_wdr_node_name;
use wdrlib::zk_status_path;

fn main() {
    env_logger::init();

    fn_info!("zk status path: {}", zk_status_path!(get_wdr_node_name()));

    manager::run();
}
