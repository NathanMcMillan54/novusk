use crate::liba64::{libdif::DIF, libkernprinter::Printer};
use core::fmt::{Arguments, Write};
use core::ptr::write_volatile;
use dif::DifFieldNames;
use novuskinc::drivers::Driver;
use super::kernel::AARCH64_KERNEL;
use super::drivers::DEVICE_DRIVERS;
use novuskinc::drivers::names::*;
use printk::Printk;

// Gets called from ``kernel/printk/``
#[no_mangle]
pub unsafe extern "C" fn _early_printk(fmt: Arguments) {
    let driver_name = DIF.get(DifFieldNames::PrintingMethod);
    let mut driver = DEVICE_DRIVERS.get_driver(driver_name);

    if driver.is_none() {
        if DIF.get(DifFieldNames::EnableSerial).parse::<bool>().unwrap_or(false) {
            let driver = DEVICE_DRIVERS.get_driver(SIMPLE_UART);

            if driver.is_some() {
                let mut printer = Printer(driver.unwrap());
                printer.write_fmt(fmt);
                return;
            } else { panic!("Can't find {} driver for printing", driver_name) }
        }
    }

    let mut printer = Printer(driver.unwrap());

    printer.write_fmt(fmt);
}

#[no_mangle]
pub extern "C" fn _kernel_main_print(fmt: Arguments) {

}

#[no_mangle]
pub static mut PRINTK: Printk = Printk {
    init: false,
    console_driver: None
};
