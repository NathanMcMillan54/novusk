#![no_std]
#![feature(asm, global_asm)]

extern crate fk_std;

pub const ARCH: &str = "x86_64";

pub mod boot;
pub mod include;
