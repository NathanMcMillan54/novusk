#![no_std]
#![no_main]
#![feature(asm)]

#[path = "../aarch64.rs"]
pub mod arch;

pub use arch::*;
