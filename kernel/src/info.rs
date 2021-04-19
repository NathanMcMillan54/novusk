use core::fmt::{Arguments, Write};
use super::printk::_print;

extern "C" {
    fn x86_time() -> f64;
}

pub unsafe fn _info(info: Arguments) -> Arguments {
    // Until other architectures are supported the time will be x86_time
    let time = x86_time();
    _print(format_args!("[ {} ] {}", time, info));
    return info;
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {$crate::info::_info(format_args!($($arg)*))};
}
