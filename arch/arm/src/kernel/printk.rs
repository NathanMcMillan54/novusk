use core::fmt::{Arguments, Result, Write};
use dif::DifFieldNames;
use novuskinc::drivers::get_driver;
use printk::{early::EarlyPrinter, Printk};
use crate::liba32::libdif::DIF;

#[no_mangle]
pub unsafe extern "C" fn _early_printk(fmt: Arguments) -> Result {
    let mut printer = EarlyPrinter(DIF.get(DifFieldNames::PrintingMethod), get_driver(DIF.get(DifFieldNames::PrintingMethod)).unwrap());

    printer.write_fmt(fmt);

    Ok(())
}

#[no_mangle]
pub static mut PRINTK: Printk = Printk {
    init: false,
    console_driver: None
};
