#![no_std]
#![feature(asm, global_asm)]

pub const ARCH: &str = "x86_64";

pub mod boot;
pub mod include;
