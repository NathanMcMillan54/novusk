#![no_std]

#[macro_use] extern crate lazy_static;

use core::fmt::Arguments;

pub mod console;

static mut KMAIN_PRINT: bool = false;

extern "C" {
    pub(crate) fn arch_printk(fmt: Arguments);
}

pub fn _printk(fmt: Arguments) -> Arguments {
    unsafe {
        if !KMAIN_PRINT {
            unsafe { arch_printk(fmt); }
        } else if KMAIN_PRINT {
            let mut kconsole = console::KernelConsole::new();
            kconsole.write_fmt(fmt);
        }
    }

    return fmt;
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {$crate::_printk(format_args!($($arg)*))};
}
