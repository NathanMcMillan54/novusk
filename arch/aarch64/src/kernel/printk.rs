use core::fmt::{Arguments, Write};
use super::uart::Uart;

#[no_mangle]
#[export_name = "arch_printk"]
pub extern "C" fn _aarch64_printk(fmt: Arguments) {
    let mut uart = Uart::new();

    uart.write_fmt(fmt);
}

#[no_mangle]
pub extern "C" fn _kernel_main_print(fmt: Arguments) {
    _aarch64_printk(fmt);
}

#[macro_export]
macro_rules! aarch64_printk {
    ($($arg:tt)*) => {$crate::kernel::printk::_aarch64_printk(format_args!($($arg)*))};
}
