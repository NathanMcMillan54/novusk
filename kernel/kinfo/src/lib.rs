#![no_std]

#[macro_use] extern crate log;

use core::fmt::Arguments;

pub mod error;

pub(crate) static mut KERN_INFO: &'static str = "ok";

pub unsafe fn _kinfo(args: Arguments) {
    if !printk_init() {
        return;
    } else {
        info!("[ {} ] {}", KERN_INFO, args);
    }
    KERN_INFO = "ok";
}

#[no_mangle]
pub unsafe extern "C" fn current_info() -> &'static str {
    return KERN_INFO;
}

#[macro_export]
macro_rules! kinfo {
    ($($arg:tt)*) => {$crate::_kinfo(format_args!($($arg)*))};
}

extern "C" {
    fn printk_init() -> bool;
}
