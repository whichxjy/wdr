#[macro_use]
extern crate fnlog;

pub mod config;
pub mod info;
pub mod zk;

use lazy_static::lazy_static;

lazy_static! {
    // zk config path
    pub static ref ZK_CONFIG_PATH: &'static str = "/config";
    // zk node path
    pub static ref ZK_NODE_PATH: &'static str = "/node";
    // zk info path
    pub static ref ZK_INFO_PATH: &'static str = "/info";
}

#[macro_export]
macro_rules! zk_node_path {
    ($node_name:expr) => {
        format!("{}/{}", &wdrlib::ZK_NODE_PATH as &str, $node_name)
    };
}

#[macro_export]
macro_rules! zk_node_info_path {
    ($node_name:expr) => {
        format!("{}/{}", &wdrlib::ZK_INFO_PATH as &str, $node_name)
    };
}
