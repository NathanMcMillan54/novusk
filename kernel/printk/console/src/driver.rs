use novuskinc::console::KernelConsoleDriver;
use novuskinc::drivers::names::{FRAME_BUFFER, SERIAL, SIMPLE_UART};
use crate::MainKernelConsole;

impl KernelConsoleDriver for MainKernelConsole {
    fn write_character(&self, c: char, x: u16, y: u16) {
        if self.printing_method.is_none() {
            panic!("Printing method for {} not found", self.console.name);
        }

        self.console.chars_written.set(self.console.chars_written.get() + 1);

        // Checks if a new line is needed
        if self.console.width.get() < self.console.chars_written.get() as u16 || c == '\n' {
            self.console.chars_written.set(0);
        }

        let driver_name = self.printing_method.unwrap().name();

        if driver_name == SERIAL || driver_name == SIMPLE_UART {
            self.serial_write_char(c);
        } else if driver_name == FRAME_BUFFER {
            self.graphics_write_char(c, x);
        }
    }

    fn write_string(&self, string: &str, x: u16, y: u16) {
        for (ix, c) in string.chars().enumerate() {
            self.write_character(c, ix as u16 + x, 0);
        }
    }

    fn new_line(&self) {
        self.console.line.set(self.console.line.get() + 1);
    }

    fn clear_screen(&self, option: u16) {
        for _ in 0..self.dimensions().1 {
            self.new_line();
        }
    }

    fn dimensions(&self) -> (u16, u16) {
        return (self.console.width.get(), self.console.height.get());
    }
}
