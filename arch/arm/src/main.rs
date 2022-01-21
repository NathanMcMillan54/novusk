#![no_std]
#![no_main]
#![feature(asm, global_asm)]
#![feature(alloc_error_handler, lang_items)]

pub(crate) extern crate rlibc;

#[path = "../arm.rs"]
pub mod arch;

pub use arch::*;

#[cfg(target_arch = "aarch64")]
pub mod bits64;
