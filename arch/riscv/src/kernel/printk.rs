use core::fmt::{Arguments, Write};
use super::uart::Uart;

#[export_name = "arch_printk"]
#[no_mangle]
pub unsafe extern "C" fn _riscv_printk(fmt: Arguments) {
    let uart = Uart::current_address();
    write!(Uart::new(uart), "{}{}", fmt, "\n");
}

#[export_name = "_kernel_main_print"]
#[no_mangle]
pub extern "C" fn _riscv_main_print(fmt: Arguments) {

}

#[macro_export]
macro_rules! riscv_printk {
    ($($arg:tt)*) => {$crate::kernel::printk::_riscv_printk(format_args!($($arg)*))};
}
