use crate::drivers::device::DEVICE_INFO;
use core::ptr::write_volatile;
use crate::drivers::virt::uart::UART;

extern "C" {
    fn early_write_byte(b: u8);
}

pub struct Uart0;

impl Uart0 {
    pub unsafe fn write_byte(&mut self, b: u8) {
        early_write_byte(b);
    }

    pub unsafe fn write_bytes(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.write_byte(*b);
        }
    }

    pub unsafe fn write_string(&mut self, string: &str) {
        self.write_bytes(string.as_bytes());
    }
}
