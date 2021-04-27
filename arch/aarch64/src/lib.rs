#![no_std]
#![feature(global_asm)]

#[cfg(target_arch = "aarch64")]
pub mod boot;
