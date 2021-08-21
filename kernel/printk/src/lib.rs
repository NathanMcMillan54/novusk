#![no_std]

use core::fmt::Arguments;

extern "C" {
    pub(crate) fn arch_printk(fmt: Arguments);
    pub(crate) fn graphics_print(x: usize, y: usize, color: usize, args: Arguments);
    static mut IN_KERNEL: bool;
}

pub fn _printk(fmt: Arguments) -> Arguments {
    unsafe {
        if !IN_KERNEL {
            arch_printk(fmt);
        } else {
            graphics_print(0, 0, 15, fmt);
        }
    }

    return fmt;
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {$crate::_printk(format_args!($($arg)*))};
}
