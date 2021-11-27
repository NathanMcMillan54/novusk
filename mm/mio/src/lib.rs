#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::{volatile_load, volatile_store};

pub unsafe fn mmio_write(reg: u32, val: u32) {
    volatile_store(reg as *mut u32, val);
}

pub unsafe fn mmio_read(reg: u32) -> u32 {
    let ret = volatile_load(reg as *const u32);

    return ret;
}
