#![no_std]

pub extern crate console;
#[macro_use] extern crate novuskinc;

use core::borrow::Borrow;
use core::fmt::{Arguments, Write};
use novuskinc::drivers::Driver;
use novuskinc::drivers::manager::DeviceDriverManager;
use crate::init::error::SUCCESS;

pub mod early;
pub mod init;
pub mod macros;

/// This module contains functions that ``kernel/tests/`` uses
#[cfg(feature = "test")]
pub mod tests;

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

    pub fn reset(&mut self) {
        let writer = self.console_driver;

        writer.unwrap().clear_screen(0);
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

pub unsafe fn printk_init(writer_driver: &'static str) -> u8 {
    PRINTK.console_driver = DEVICE_DRIVERS.get_driver(writer_driver);
    PRINTK.init = true;

    SUCCESS
}
