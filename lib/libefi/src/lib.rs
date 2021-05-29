#![no_std]

pub mod power;

use core::ptr::NonNull;
use uefi::table::{Boot, SystemTable};

extern "C" {
    pub fn st() -> NonNull<SystemTable<Boot>>;
}
