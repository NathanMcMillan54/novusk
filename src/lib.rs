#![no_std]
#![crate_type = "staticlib"]
#![feature(asm)]

pub extern crate drivers;
pub extern crate include;
extern crate init;
#[macro_use]
pub extern crate kernel;
pub extern crate libn;

#[cfg(target_arch = "x86_64")]
extern crate x86_64;

mod panic;
