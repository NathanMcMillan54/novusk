#![no_std]

extern "C" {
    pub fn mmio_write(reg: u64, val: u32);
    pub fn mmio_read(reg: i64) -> u64;
}

