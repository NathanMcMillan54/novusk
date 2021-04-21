use core::fmt::Arguments;

use include::novusk::syscalls::{kernel_time, write_fmt};

pub unsafe fn _printk(args: Arguments) -> Arguments {
    write_fmt(format_args!("{}{}", args, "\n"));
    kernel_time();
    return args;
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {$crate::printk::_printk(format_args!($($arg)*))};
}
