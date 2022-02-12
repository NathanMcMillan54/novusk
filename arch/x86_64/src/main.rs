#![no_std]
#![no_main]
#![feature(asm)]
#![feature(alloc_error_handler, panic_info_message)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate novuskinc;
pub(crate) extern crate memory;

#[path = "../x86_64.rs"]
pub mod arch;

pub use arch::*;
