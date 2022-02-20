use core::fmt::{Arguments, Write};
use novuskinc::serial::{SerialIo, KERNEL_SERIALIO};
use kernel::KERNEL;
use crate::include::dif::DIF;

pub static mut AARCH64_SERIALIO: SerialIo = SerialIo::empty();

#[no_mangle]
pub unsafe extern "C" fn _early64_printk(fmt: Arguments) {
    AARCH64_SERIALIO.write_fmt(fmt);
}

#[no_mangle]
pub unsafe extern "C" fn aarch64_setup_early_printk() {
    //AARCH64_SERIALIO.serial_addr = 0x3F20_1000 as *mut u8;
    AARCH64_SERIALIO.serial_addr = DIF.serial_addr.unwrap() as *mut u8;
    KERNEL.set_serial(AARCH64_SERIALIO);
}

#[macro_export]
macro_rules! early_printk {
    ($($arg:tt)*) => { unsafe { $crate::kernel::early_printk::_early64_printk(format_args!($($arg)*)); } }
}
