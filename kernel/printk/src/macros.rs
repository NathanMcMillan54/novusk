use core::fmt::{Arguments, Result, Write};
use crate::{can_printk_work, DEVICE_DRIVERS, PRINTK};
use novuskinc::drivers::names::CONSOLE;
use crate::early::_early_printk;

pub fn _printk(fmt: Arguments) -> Result {
    unsafe {
        if can_printk_work() {
            PRINTK.write_fmt(fmt);
            Ok(())
        } else { _early_printk(fmt) }
    }
}

#[macro_export]
macro_rules! printk {
    ($($args:tt)*) => {$crate::macros::_printk(format_args!($($args)*))};
}

#[macro_export]
macro_rules! early_printk {
    ($($args:tt)*) => {unsafe { $crate::early::_early_printk(format_args!($($args)*)) } };
}
