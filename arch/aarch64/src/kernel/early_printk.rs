use core::fmt::{Arguments, Write};
use kernel::KERNEL;
use crate::include::dif::DIF;
use super::uart::KERNEL_SIMPLEUART;

#[no_mangle]
pub unsafe extern "C" fn _early64_printk(fmt: Arguments) {
   KERNEL_SIMPLEUART.write_fmt(fmt);
}

#[no_mangle]
pub unsafe extern "C" fn aarch64_setup_early_printk() {

}

#[macro_export]
macro_rules! early_printk {
    ($($arg:tt)*) => { unsafe { $crate::kernel::early_printk::_early64_printk(format_args!($($arg)*)); } }
}
