use core::fmt::{Arguments, Write};
use crate::kernel::uart::Uart;

#[export_name = "arch_printk"]
#[no_mangle]
pub extern "C" fn _a64_printk(fmt: Arguments) {
    let mut uart = Uart::new();
    uart.write_fmt(format_args!("{}", fmt));
}

#[export_name = "_kernel_main_print"]
#[no_mangle]
pub extern "C" fn _a64_main_printk(fmt: Arguments) {
    // For now just call the "early" printk function
    _a64_printk(fmt);
}

#[macro_export]
macro_rules! aarch64_printk {
    ($($arg:tt)*) => {$crate::kernel::printk::_a64_printk(format_args!($($arg)*))};
}
