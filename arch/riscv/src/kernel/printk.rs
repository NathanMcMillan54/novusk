use core::borrow::{Borrow, BorrowMut};
use core::fmt::{Arguments, Write};
use printk::Printk;
use crate::kernel::platform::RISCV_DEVICE;

#[no_mangle]
pub extern "C" fn _early_printk(fmt: Arguments) -> core::fmt::Result {
    let mut printer = RvEarlyPrintk;

    printer.write_fmt(fmt)
}

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
pub static mut PRINTK: Printk = Printk {
    init: false,
    console_driver: None
};
