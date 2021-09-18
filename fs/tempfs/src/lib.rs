#![no_std]

use core::ptr::{read_volatile, write_volatile};

pub mod file;

pub struct TempFs {
    pub name: &'static str,
    pub mem_address: *const u8,
}

impl TempFs {
    pub fn open(file_name: &'static str) -> Self {
        let mut mem_addr = file_name.as_ptr();

        return TempFs { name: file_name, mem_address: mem_addr };
    }

    pub fn read(&mut self) -> u8 {
        unsafe { return read_volatile(self.mem_address); }
    }

    pub fn write_byte(&mut self, buf: u8) {
        unsafe { write_volatile(buf as *mut u8, *self.mem_address); }
    }
}
