#![no_std]

#[macro_use] extern crate printk;

use core::fmt::Arguments;

pub mod info;

pub unsafe fn _kinfo(msg: Arguments) {
    printk!("INFO [ {} ] {}", info::KINFO, msg);
}

#[macro_export]
macro_rules! kinfo {
    ($($arg:tt)*) => {$crate::_kinfo(format_args!($($arg)*))};
}
