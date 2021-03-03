#[macro_use]
use crate::aarch64lib::print::{_aarch64_print};
use crate::kernel::{time::time::kernel_time};
use core::fmt::Arguments;

pub fn _e_kinfo(info: Arguments) {
    unsafe {
        _aarch64_print(format_args!("[ {} ] {}", kernel_time(), info));
    }
}

#[macro_export]
macro_rules! e_kinfo {
    ($($arg:tt)*) => {$crate::kernel::early_info::_e_kinfo(format_args!($($arg)*))};
}