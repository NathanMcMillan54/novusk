#![no_std]
#![crate_type = "staticlib"]
#![feature(asm)]

extern crate drivers;
extern crate kernel;
pub use kernel::die;
extern crate x86_64;

mod panic;
