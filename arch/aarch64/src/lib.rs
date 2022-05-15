#![no_std]
#![feature(asm, global_asm)]
#![feature(panic_info_message)]
#![allow(warnings)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate novuskinc;
//pub(crate) extern crate arm;

#[path = "../aarch64.rs"]
pub mod arch;

pub use arch::*;
