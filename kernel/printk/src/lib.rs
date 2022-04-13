#![no_std]

#[macro_use] extern crate novuskinc;

use core::fmt::{Arguments, Write};
use novuskinc::drivers::manager::DeviceDriverManager;

pub mod macros;

extern "C" {
    pub(crate) static mut DEVICE_DRIVERS: DeviceDriverManager;
}

pub struct Printk {
    pub writer: &'static str,
}

impl Printk {
    pub const fn new() -> Self {
        return Printk {
            writer: "None",
        };
    }
}

impl Write for Printk {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let writer = unsafe { DEVICE_DRIVERS.get_driver(self.writer) };

        if writer.is_none() {
            return Err(Default::default());
        } else { writer.unwrap().write_string(s, 0, 0); }

        return Ok(());
    }
}

#[no_mangle]
pub static mut PRINTK: Printk = Printk::new();

pub(crate) unsafe  fn can_printk_work() -> bool {
    if PRINTK.write_str("").is_err() {
        return false;
    }

    return true;
}

pub unsafe fn printk_init(writer_driver: &'static str) {
    for d in 0..DEVICE_DRIVERS.drivers.unwrap().len() {
        extern "C" {
            fn _early_printk(fmt: Arguments);
        }

        _early_printk(format_args!("{}{}{}", "Driver: ", DEVICE_DRIVERS.drivers.unwrap()[d].name(), "\n"));
    }
    if DEVICE_DRIVERS.get_driver(writer_driver).is_some() {
        PRINTK.writer = writer_driver;
    } else {
        panic!("{} is not a driver option to support printk, use \"Console Driver\" or \"Graphics Driver\"", writer_driver);
    }
}
