#![feature(asm, global_asm)]
#![no_std]

pub mod x86;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const ARCH: &'static str = "x86";

