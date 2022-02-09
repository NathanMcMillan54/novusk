#![no_std]
#![no_main]
#![feature(asm)]
#![allow(warnings)]

#[macro_use] extern crate alloc;

#[path = "../aarch64.rs"]
pub mod arch;

pub use arch::*;
