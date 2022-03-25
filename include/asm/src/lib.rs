#![no_std]
#![feature(asm, global_asm)]

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "arm")]
pub mod arm32;