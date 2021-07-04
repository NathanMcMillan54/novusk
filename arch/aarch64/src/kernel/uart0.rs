use core::fmt::{Arguments, Result, Write};
use core::ptr::{write_volatile};

pub struct Uart0 {
    pub address: *mut u8,
}

impl Uart0 {
    pub unsafe fn write_byte(&self, byte: u8) {
        write_volatile(self.address as *mut u8, byte)
    }

    pub unsafe fn write_bytes(&self, bytes: &[u8]) {
        for byte in bytes {
            write_volatile(self.address as *mut u8, *byte);
        }
    }

    pub unsafe fn write_string(&self, string: &str) {
        self.write_bytes(string.as_bytes());
    }
}

impl Write for Uart0 {
    fn write_str(&mut self, s: &str) -> Result {
        unsafe { self.write_string(s); }
        Ok(())
    }
}
