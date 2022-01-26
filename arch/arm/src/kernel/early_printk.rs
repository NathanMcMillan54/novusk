use core::fmt::Arguments;
use super::{serial, uart};

#[no_mangle]
pub extern "C" fn _early_printk(print: Arguments) {
    serial::hio_write(print);
}

pub fn setup_early_printk() {

}

#[macro_export]
macro_rules! early_printk {
    ($($arg:tt)*) => { $crate::kernel::early_printk::_early_printk(format_args!($($arg)*)); }
}
