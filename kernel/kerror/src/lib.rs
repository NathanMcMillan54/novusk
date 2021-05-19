#![no_std]

#[macro_use] extern crate log;

use core::fmt::{Arguments, Write};

extern "C" {
    fn info_error() -> &'static str;
    fn current_info() -> &'static str;
}

pub unsafe fn _kerror(error: Arguments) {
    kerror();
    error!("[ {} ] {}", current_info(), error);
}

#[macro_export]
macro_rules! kerror {
    ($($arg:tt)*) => {$crate::_kerror(format_args!($($arg)*))};
}

pub unsafe fn kerror() {
    info_error();
}
