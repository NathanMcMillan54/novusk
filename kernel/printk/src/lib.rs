#![no_std]

use core::fmt::{Arguments, Write};
use core::ptr::NonNull;
use uefi::prelude::{Boot, SystemTable};

extern "C" {
    fn printk_init() -> bool;
    fn st() -> NonNull<SystemTable<Boot>>;
}

pub unsafe fn _printk(args: Arguments) {
    if printk_init() {
        let stdout = st().as_ref().stdout();
        writeln!(stdout, "{}", args);
    } else { return; }
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {$crate::_printk(format_args!($($arg)*))};
}
