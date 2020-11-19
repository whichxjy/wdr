#[macro_export]
macro_rules! wdr_info {
    ($x:expr $(, $($y:expr),+)?) => ({
        info!(concat!("[{}] ", $x), function_name!() $(, $($y),+)?);
    });
}
