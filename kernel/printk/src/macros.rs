use core::fmt::{Arguments, Result, Write};
use crate::{can_printk_work, DEVICE_DRIVERS, PRINTK};
use novuskinc::drivers::names::CONSOLE;

pub fn _printk(fmt: Arguments) -> Result {
    unsafe {
        if can_printk_work() {
            PRINTK.write_fmt(fmt);
            Ok(())
        } else { Err(Default::default()) }
    }
}

#[macro_export]
macro_rules! printk {
    ($($args:tt)*) => {$crate::macros::_printk(format_args!($($args)*))};
}
