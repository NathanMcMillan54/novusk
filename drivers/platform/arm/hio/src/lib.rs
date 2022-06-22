#![no_std]
#![feature(asm, global_asm)]

pub mod driver;
pub mod io;
pub(crate) mod syscall;

pub struct HioDriver;
