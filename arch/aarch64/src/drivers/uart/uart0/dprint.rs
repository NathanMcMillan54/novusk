use core::fmt::{Arguments, Write, Result};
use super::{Uart0, UART0};

pub fn _dprint(args: Arguments) {
    let mut writer = Uart0;
    writer.write_fmt(format_args!("{}", args));
}

#[macro_export]
macro_rules! dprint {
    ($($arg:tt)*) => {$crate::drivers::uart::uart0::dprint::_dprint(format_args!($($arg)*))};
}
