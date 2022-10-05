use crate::liba64::libdif::DIF;
use core::fmt::{Arguments, Write};
use core::ptr::write_volatile;
use dif::DifFieldNames;
use novuskinc::drivers::Driver;
use super::kernel::AARCH64_KERNEL;
use super::drivers::DEVICE_DRIVERS;
use novuskinc::drivers::names::*;
use printk::Printk;

struct Printer(&'static dyn Driver);

impl Write for Printer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if unsafe { DIF.get(DifFieldNames::PrintingMethod) } == "Serial" {
            for b in s.as_bytes() {
                self.0.write(*b);
            }
        } else {  }

        Ok(())
    }
}

// Gets called from ``kernel/printk/``
#[no_mangle]
pub unsafe extern "C" fn _early_printk(fmt: Arguments) {
    /* if DIF.get(DifFieldNames::PrintingMethod) == "Serial" {
        if AARCH64_KERNEL.early {*/
    let mut driver = DEVICE_DRIVERS.get_driver(SIMPLE_UART);
    let mut printer = Printer(driver.unwrap());

    printer.write_fmt(fmt);

    /*if driver.is_none() {
        for b in "Driver not found\n".as_bytes() {
            write_volatile(0x3F20_1000 as *mut u8, *b);
        }
    }

    let string = &*format!("{:?}", fmt);

    for byte in string.as_bytes() {
        driver.unwrap().write(*byte);
    }
*/
        /* } else {
            /*let mut driver = DRIVER_MANAGER.get_driver(SERIAL);

            driver.unwrap().write_fmt(fmt);*/
        }
    } else {  }*/
}

#[no_mangle]
pub extern "C" fn _kernel_main_print(fmt: Arguments) {

}

#[no_mangle]
pub static mut PRINTK: Printk = Printk {
    init: false,
    console_driver: None
};
