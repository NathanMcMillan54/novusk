#![no_std]

use core::fmt::Arguments;

extern "C" {
    fn arch_printk(fmt: Arguments);
}

pub unsafe fn _printk(fmt: Arguments) -> Arguments {
    arch_printk(fmt);
    return fmt;
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {$crate::_printk(format_args!($($arg)*))};
}
