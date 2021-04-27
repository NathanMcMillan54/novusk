#![feature(global_asm)]
#![no_std]

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod boot;
