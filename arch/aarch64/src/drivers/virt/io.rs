use core::ptr::write_volatile;
use super::uart::{UART};

pub struct VirtWriter;

impl VirtWriter {
    pub unsafe fn write_byte(&mut self, byte: u8) -> u8 {
        write_volatile(UART, byte);
        return byte;
    }

    pub unsafe fn write_string(&mut self, string: &str) {
        for chars in string.bytes() {
            self.write_byte(chars);
        }
    }
}
