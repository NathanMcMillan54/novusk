#![no_std]

#[macro_use] extern crate kerror;
#[macro_use] extern crate kinfo;

use core::ptr::NonNull;
use uefi::table::{Boot, SystemTable};

extern "C" {
    fn st() -> NonNull<SystemTable<Boot>>;
}

pub unsafe fn fat_init() {
    // This means BootServices and nothing else!
    let bs = st().as_ref().boot_services();
}
