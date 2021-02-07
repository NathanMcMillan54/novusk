#![macro_use]
use core::fmt::{Arguments, Write};
use crate::x86lib::print::WRITER;
use super::time_init;

pub fn _e_kinfo(args: Arguments) {
    write!(WRITER.lock(), "[ {} ] {}", unsafe { time_init() }, args);
}

#[macro_export]
macro_rules! e_kinfo {
    ($($arg:tt)*) => {$crate::kernel::early_info::_e_kinfo(format_args!($($arg)*))};
}
