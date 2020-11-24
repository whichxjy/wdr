#[macro_use]
extern crate fnlog;

pub mod config;
pub mod zk;

use lazy_static::lazy_static;

lazy_static! {
    // zk config path
    pub static ref ZK_CONFIG_PATH: &'static str = "/config";
}

#[macro_export]
macro_rules! zk_status_path {
    ($node_name:expr) => {
        format!("/status/{}", $node_name)
    };
}
