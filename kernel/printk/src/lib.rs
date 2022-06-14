#![no_std]

#[macro_use] extern crate novuskinc;

use core::fmt::{Arguments, Write};
use novuskinc::drivers::Driver;
use novuskinc::drivers::manager::DeviceDriverManager;

pub mod init;
pub mod macros;

extern "C" {
    pub(crate) static mut DEVICE_DRIVERS: DeviceDriverManager;
    pub(crate) static mut PRINTK: Printk;
}

pub struct Printk {
    pub init: bool,
    pub console_driver: Option<&'static dyn Driver>,
}

impl Printk {
    pub fn set_init(&mut self, init: bool, console_driver: &'static dyn Driver) {
        self.init = init;
        self.console_driver = Some(console_driver);
    }
}

impl Write for Printk {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let writer = self.console_driver;

        if writer.is_none() {
            return Err(Default::default());
        } else { writer.unwrap().write_string(s, 0, 0); }

        return Ok(());
    }
}

pub(crate) unsafe  fn can_printk_work() -> bool {
    if PRINTK.write_str("").is_err() {
        return false;
    }

    return true;
}

pub unsafe fn printk_init(writer_driver: &'static str) {
    if DEVICE_DRIVERS.get_driver(writer_driver).is_some() {
        // PRINTK.writer = writer_driver;
    } else {
        panic!("{} is not a driver option to support printk, use \"Console Driver\" or \"Graphics Driver\"", writer_driver);
    }
}
