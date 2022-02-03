use core::fmt::{Arguments, Result, Write};
use core::ptr::{read_volatile, write_volatile};

pub static mut KERNEL_SERIALIO: SerialIo = SerialIo::empty();

pub struct SerialIo {
    pub serial_addr: *mut u8,
}

impl SerialIo {
    pub fn new() -> Self {
        return SerialIo {
            serial_addr: 0x0 as *mut u8,
        };
    }

    pub const fn empty() -> Self {
        return SerialIo {
            serial_addr: 0x0 as *mut u8,
        };
    }

    pub fn set_serial_addr(&mut self, address: *mut u8) {
        self.serial_addr = address;
    }

    pub unsafe fn serial_write_byte(&self, byte: u8) {
        write_volatile(self.serial_addr, byte);
    }

    pub unsafe fn serial_read_byte(&self) -> u8 {
        return read_volatile(self.serial_addr);
    }
}

impl Write for SerialIo {
    fn write_str(&mut self, s: &str) -> Result {
        for byte in s.as_bytes() {
            unsafe { self.serial_write_byte(*byte); }
        }

        Ok(())
    }
}

