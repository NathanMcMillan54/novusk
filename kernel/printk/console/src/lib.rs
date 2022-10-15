#![no_std]

#[macro_use] extern crate novuskinc;

use core::cell::Cell;
use core::fmt::Arguments;
use dif::{Dif, DifFieldNames};
use novuskinc::console::KernelConsole;
use novuskinc::drivers::{Driver, manager::DeviceDriverManager, names::*};

pub mod driver;

#[no_mangle]
pub unsafe extern "C" fn console_init() -> u8 {
    extern "C" {
        static mut DIF: Dif;
        static mut DEVICE_DRIVERS: DeviceDriverManager;
    }

    let printing_method = DIF.get(DifFieldNames::PrintingMethod);

    let driver = match printing_method {
        "Serial" | SERIAL => DEVICE_DRIVERS.get_driver(SERIAL),
        "Frame Buffer" | FRAME_BUFFER => DEVICE_DRIVERS.get_driver(FRAME_BUFFER),
        _ => DEVICE_DRIVERS.get_driver(SIMPLE_UART),
    };

    KERNEL_CONSOLE.printing_method = driver;

    0
}

pub struct MainKernelConsole {
    pub console: KernelConsole,
    pub printing_method: Option<&'static dyn Driver>,
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

#[no_mangle]
pub static mut KERNEL_CONSOLE: MainKernelConsole = MainKernelConsole::new();
