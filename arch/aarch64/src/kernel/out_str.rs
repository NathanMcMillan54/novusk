use core::{fmt, ptr};
use crate::drivers::uart;

pub struct Writer;

impl Writer {
    pub fn write_byte(&mut self, arg: &[u8]) {
        for byte in arg {
            unsafe { ptr::write_volatile(uart::UART0, *byte) }
        }
    }
}

impl Writer {
    pub fn write_string(&mut self, arg: &str) {
        let byte = arg.as_bytes();
        self.write_byte(byte);
    }
}

impl Writer {
    pub fn new_line(&mut self) {
        self.write_byte(b"\n");
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, args: &str) -> fmt::Result {
        self.write_string(args);
        Ok(())
    }
}
