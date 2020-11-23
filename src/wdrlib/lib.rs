pub mod model;

use log::*;
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
