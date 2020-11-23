use log::*;
use serde::{Deserialize, Serialize};
use stdext::*;

#[macro_export]
macro_rules! wdr_trace {
    ($x:expr $(, $($y:expr),+)?) => {
        trace!(concat!("[{}] ", $x), function_name!() $(, $($y),+)?);
    };
}

#[macro_export]
macro_rules! wdr_debug {
    ($x:expr $(, $($y:expr),+)?) => {
        debug!(concat!("[{}] ", $x), function_name!() $(, $($y),+)?);
    };
}

#[macro_export]
macro_rules! wdr_info {
    ($x:expr $(, $($y:expr),+)?) => {
        info!(concat!("[{}] ", $x), function_name!() $(, $($y),+)?);
    };
}

#[macro_export]
macro_rules! wdr_warn {
    ($x:expr $(, $($y:expr),+)?) => {
        warn!(concat!("[{}] ", $x), function_name!() $(, $($y),+)?);
    };
}

#[macro_export]
macro_rules! wdr_error {
    ($x:expr $(, $($y:expr),+)?) => {
        error!(concat!("[{}] ", $x), function_name!() $(, $($y),+)?);
    };
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct WdrConfig {
    pub configs: Vec<ProcessConfig>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct ProcessConfig {
    pub name: String,
    pub version: String,
    pub resource: String,
    pub cmd: String,
}

impl WdrConfig {
    pub fn from_str(data: &str) -> Option<Self> {
        let wdr_config: WdrConfig = match serde_json::from_str(data) {
            Ok(wdr_config) => wdr_config,
            Err(err) => {
                wdr_error!("Fail to parse wdr config: {}", err);
                return None;
            }
        };

        Some(wdr_config)
    }
}
