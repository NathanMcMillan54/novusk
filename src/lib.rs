#![no_std]
#![crate_type = "staticlib"]
#![feature(alloc_error_handler, asm)]

pub extern crate drivers;
pub extern crate include;
pub extern crate fs;
extern crate init;
#[macro_use]
pub extern crate kernel;
pub extern crate libn;
pub extern crate mm;

#[cfg(target_arch = "x86_64")]
extern crate x86_64;

#[cfg(target_arch = "aarch64")]
extern crate aarch64;
