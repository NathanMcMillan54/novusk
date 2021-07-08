use core::ptr::{read_volatile, write_volatile};

pub trait Uart {
    unsafe fn init(&self) {
        // Board specific setup
    }

    unsafe fn write_byte(&self, byte: u8) {
        // How ever you want to print
        write_volatile(0x0 as *mut u8, byte);
    }

    unsafe fn write_bytes(&self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(*byte);
        }
    }

    unsafe fn write_string(&self, string: &str) {
        self.write_bytes(string.as_bytes());
    }

    unsafe fn read_byte(&self) -> u8 {
        // Or however you read from Uart
        return read_volatile(0x0 as *mut u8);
    }

    unsafe fn read_bytes(&self) -> &[u8] {
        self.read_byte();
        b""
    }
}
