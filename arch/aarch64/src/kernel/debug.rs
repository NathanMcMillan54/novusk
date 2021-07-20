// For debug printing in Qemu
use core::fmt::{Result, Write};
use core::ptr::write_volatile;

pub struct DebugPrint;

impl DebugPrint {
    pub unsafe fn write_byte(&mut self, byte: u8) {
        write_volatile(0x3F20_1000 as *mut u8, byte);
    }

    pub unsafe fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(*byte);
        }
    }

    pub unsafe fn write_string(&mut self, string: &str) {
        self.write_bytes(string.as_bytes());
    }
}

impl Write for DebugPrint {
    fn write_str(&mut self, str: &str) -> Result {
        unsafe { self.write_string(str); }
        Ok(())
    }
}
