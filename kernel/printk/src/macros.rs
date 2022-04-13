use core::fmt::{Arguments, Write};

pub fn _printk(fmt: Arguments) {
    unsafe { crate::PRINTK.write_fmt(fmt); }
}

#[macro_export]
macro_rules! printk {
    ($($args:tt)*) => {$crate::macros::_printk(format_args!($($args)*))};
}
