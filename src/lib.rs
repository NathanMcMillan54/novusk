#![no_std]
#![crate_type = "staticlib"]
#![feature(asm)]

#[cfg(target_arch = "arm")]
extern crate arm;

extern crate drivers;
extern crate kernel;
pub use kernel::die;

#[cfg(target_arch = "x86_64")]
extern crate x86_64;

mod panic;
