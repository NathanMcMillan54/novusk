use core::fmt::{Arguments, Write};

extern "C" { pub fn _x86_print(args: Arguments); }

pub unsafe fn _print(args: Arguments) {
    #[cfg(target_arch = "x86_64")]
    _x86_print(args);
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {$crate::printk::_print(format_args!($($arg)*))};
}
