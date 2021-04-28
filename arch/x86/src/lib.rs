#![no_std]
#![feature(asm, global_asm)]

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod boot;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod drivers;
