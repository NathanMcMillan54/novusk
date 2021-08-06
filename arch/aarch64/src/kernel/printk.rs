use core::fmt::{Arguments, Write};
use super::debug::DebugPrint;

#[export_name = "arch_printk"]
#[no_mangle]
pub extern "C" fn _a64_printk(fmt: Arguments) {
    let mut debug = DebugPrint;

    debug.write_fmt(fmt);
}

#[macro_export]
macro_rules! aarch64_printk {
    ($($arg:tt)*) => {$crate::kernel::printk::_a64_printk(format_args!($($arg)*))};
}
