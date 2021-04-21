use core::fmt::Arguments;
use include::novusk::syscalls::{kernel_time, write_fmt};

pub unsafe fn _kinfo(info: Arguments) -> Arguments {
    let time = kernel_time();
    write_fmt(format_args!("[ {} ] {}{}", kernel_time(), info, "\n"));
    return info;
}

#[macro_export]
macro_rules! kinfo {
    ($($arg:tt)*) => {$crate::kinfo::_kinfo(format_args!($($arg)*))};
}
