use core::fmt::{Arguments, Result, Write};
use core::ptr::write_volatile;
use novuskinc::drivers::{Driver, DriverResult};
use novuskinc::prelude::*;

pub const UART0: *mut u8 = 0x3F20_1000 as *mut u8;

#[derive(Copy, Clone)]
pub struct Rpi3Uart {
    pub debug: bool,
    pub console: KernelConsole,
}

impl Rpi3Uart {
    pub fn new() -> Self {
        return Rpi3Uart {
            debug: true, // Just for now
            console: KernelConsole::new("RPi3 UART console",128,64),
        };
    }

    pub fn uart_write_byte(&self, b: u8) {

    }

    pub fn uart_debug_write_byte(&self, b: u8) {
        unsafe {
            write_volatile(UART0, b);
        }
    }
}

impl Write for Rpi3Uart {
    fn write_str(&mut self, s: &str) -> Result {
        for b in s.as_bytes() {
            if self.debug {
                self.uart_debug_write_byte(*b);
            } else { self.uart_write_byte(*b); }
        }

        Ok(())
    }
}

impl KernelConsoleDriver for Rpi3Uart {
    fn write_character(&mut self, c: char, x: u16, y: u16) {
        self.uart_write_byte(c as u8);
        self.console.chars_written += 1;
    }

    fn write_string(&mut self, string: &str, x: u16, y: u16) {
        for b in string.as_bytes() {
            self.write_character(*b as char, x, y);
        }
    }

    fn clear_screen(&mut self, option: u16) {
        self.new_line();
        self.new_line();
    }

    fn dimensions(&mut self) -> (u16, u16) {
        return (self.console.width, self.console.height);
    }
}

impl FrameBufferGraphics for Rpi3Uart {}

impl KeyboardInput for Rpi3Uart {}

impl Driver for Rpi3Uart {
    fn name(&self) -> &'static str {
        return "Console Driver";
    }

    fn init(&self) -> Option<DriverResult> {
        return Some(Ok(()));
    }
}
