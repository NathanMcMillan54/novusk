#![no_std]
#![feature(const_mut_refs)]

#[macro_use] extern crate novuskinc;

use core::borrow::BorrowMut;
use core::cell::Cell;
use core::fmt::Arguments;
use core::ops::Deref;
use dif::{Dif, DifFieldNames};
use novuskinc::console::KernelConsole;
use novuskinc::drivers::{Driver, manager::DeviceDriverManager, names::*};

pub mod driver;

#[no_mangle]
pub unsafe extern "C" fn console_init() -> u8 {
    0
}

pub struct MainKernelConsole {
    pub console: KernelConsole,
    pub printing_method: Option<&'static mut dyn Driver>,
}

impl MainKernelConsole {
    pub const fn new() -> Self {
        return MainKernelConsole {
            console: KernelConsole {
                name: "Main Kernel Console",
                width: Cell::new(0),
                height: Cell::new(0),
                line: Cell::new(0),
                chars_written: Cell::new(0),
            },
            printing_method: None,
        };
    }

    pub fn set_width_height(&mut self, width: u16, height: u16) {
        self.console.width.set(width);
        self.console.height.set(height);
    }

    fn serial_write_char(&self, c: char) {
        self.printing_method.as_ref().unwrap().write(c as u8);
    }

    fn graphics_write_char(&self, c: char, x: u16) {
        let mut driver = &self.printing_method;

        match driver {
            Some(d) => {
                d.write_character(c, x, self.console.line.get());
            }
            _ => {},
        }
        //driver.write_character(c, x, self.console.line.get());
    }
}

#[no_mangle]
pub static mut KERNEL_CONSOLE: MainKernelConsole = MainKernelConsole::new();
