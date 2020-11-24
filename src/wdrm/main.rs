#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate fnlog;

mod manager;
mod setting;

fn main() {
    env_logger::init();

    match manager::write_config() {
        Ok(()) => fn_info!("Success"),
        Err(err) => fn_error!("Fail to write config: {}", err),
    }
}
