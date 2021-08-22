#![no_std]

use core::fmt::Arguments;

pub mod kmain;

pub static mut EARLY_PRINTK: bool = true;

extern "C" {
    pub(crate) fn arch_printk(fmt: Arguments);
    pub(crate) fn kmain_print(args: Arguments);
}

pub unsafe fn change_printk() {
    EARLY_PRINTK = false;
}

pub fn _printk(fmt: Arguments) -> Arguments {
    unsafe {
        if EARLY_PRINTK {
            arch_printk(fmt);
        } else if !EARLY_PRINTK {
            kmain_print(fmt);
        } else {
            return fmt;
        }
    }

    return fmt;
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {$crate::_printk(format_args!($($arg)*))};
}
