use core::fmt::{Arguments, Write};
use super::info::x86_time;

extern "C" { pub fn _x86_print(args: Arguments); }

pub unsafe fn _print(args: Arguments) {
    #[cfg(target_arch = "x86_64")]
    _x86_print(args);
    x86_time();
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {$crate::printk::_print(format_args!($($arg)*))};
}
