#![no_std]
#![feature(asm)]
#![feature(abi_x86_interrupt, alloc_error_handler, panic_info_message)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate novuskinc;
pub(crate) extern crate memory;
pub(crate) extern crate vgag;

#[path = "../x86_64.rs"]
pub mod arch;

pub use arch::*;
