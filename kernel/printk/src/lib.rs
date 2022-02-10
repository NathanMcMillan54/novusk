#![no_std]

#[macro_use] extern crate alloc;

use core::fmt::Arguments;

static mut KMAIN_PRINT: bool = false;

extern "C" {
    pub(crate) fn _early_printk(fmt: Arguments);
    pub(crate) fn kmain_printk(fmt: Arguments);
}

pub fn _printk(fmt: Arguments) -> Arguments {
    unsafe { _early_printk(fmt); }
    return fmt;
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {$crate::_printk(format_args!($($arg)*))};
}
