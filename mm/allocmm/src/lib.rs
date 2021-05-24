#![no_std]

use core::ptr::NonNull;
use uefi::table::{Boot, SystemTable};

extern "C" {
    fn st() -> NonNull<SystemTable<Boot>>;
}

pub unsafe fn allocmm_init() {
    let bt = st().as_ref().boot_services();
    uefi::alloc::init(bt);
}
