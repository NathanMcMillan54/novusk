use core::borrow::{Borrow, BorrowMut};
use core::fmt::{Arguments, Write};
use crate::kernel::platform::RISCV_DEVICE;

struct RvEarlyPrintk;

impl Write for RvEarlyPrintk {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            if RISCV_DEVICE.console.is_none() {
                return Err(Default::default());
            }

            RISCV_DEVICE.console.unwrap().write_string(s, 0, 0);

            return Ok(());
        }
    }
}

#[no_mangle]
#[export_name = "arch_printk"]
pub fn _rv_printk(fmt: Arguments) -> Arguments {
    let mut printer = RvEarlyPrintk;

    printer.write_fmt(fmt);

    return fmt;
}

#[no_mangle]
pub extern "C" fn kmain_printk(fmt: Arguments) {
    _rv_printk(fmt);
}

#[macro_export]
macro_rules! rv_printk {
    ($($arg:tt)*) => {$crate::kernel::printk::_rv_printk(format_args!($($arg)*))};
}
