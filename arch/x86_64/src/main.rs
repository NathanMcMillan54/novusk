#![no_std]
#![no_main]
#![feature(asm)]
#![feature(panic_info_message)]

#[macro_use] extern crate lazy_static;

#[path = "../x86_64.rs"]
pub mod arch;

pub use arch::*;
