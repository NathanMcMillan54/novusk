use core::fmt::{Result, Write};
use core::ptr::write_volatile;
use device::Device;

pub struct Rpi2Uart {
    pub address: *mut u8,
}

impl Rpi2Uart {
    pub fn new() -> Self {
        return Rpi2Uart { address: 0x3F20_1000 as *mut u8 };
    }

    pub fn write_byte(&self, byte: u8) {
        unsafe { write_volatile(self.address, byte); }
    }

    pub fn write_bytes(&self, bytes: &[u8]) {
        for b in bytes {
            self.write_byte(*b);
        }
    }
}

impl Write for Rpi2Uart {
    fn write_str(&mut self, string: &str) -> Result {
        self.write_bytes(string.as_bytes());

        Ok(())
    }
}
