#![no_std]
#![crate_type = "staticlib"]
#![feature(asm)]

extern crate drivers;
extern crate include;
extern crate init;
extern crate kernel;
pub use kernel::die;

#[cfg(target_arch = "x86_64")]
extern crate x86_64;

mod panic;
