#![no_std]

#[macro_use] extern crate novuskinc;

use core::cell::Cell;
use novuskinc::console::KernelConsole;
use novuskinc::drivers::Driver;

pub mod driver;

pub struct MainKernelConsole {
    console: KernelConsole,
    printing_method: Option<&'static dyn Driver>,
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
        self.printing_method.unwrap().write(c as u8);
    }

    fn graphics_write_char(&self, c: char, x: u16) {
        self.printing_method.unwrap().write_character(c, x, self.console.line.get());
    }
}
