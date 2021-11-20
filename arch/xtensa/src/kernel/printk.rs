use core::fmt::Arguments;
use device::Device;
use crate::board::get_board;

pub fn _xprintk(fmt: Arguments) {
    get_board().write_bytes(fmt.as_str().unwrap().as_bytes());
}

#[macro_export]
macro_rules! xprintk {
    ($($arg:tt)*) => {$crate::kernel::printk::_xprintk(format_args!($($arg)*))};
}
