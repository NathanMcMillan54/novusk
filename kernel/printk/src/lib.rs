#![no_std]

use core::fmt::Arguments;

// For the main kernel
pub mod kernel;

extern "C" {
    fn arch_printk(fmt: Arguments);
}

pub fn _printk(fmt: Arguments) -> Arguments {
    unsafe { arch_printk(fmt); }
    return fmt;
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {$crate::_printk(format_args!($($arg)*))};
}
