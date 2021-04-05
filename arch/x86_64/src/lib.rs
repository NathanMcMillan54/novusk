#![no_std]
#![feature(asm, global_asm, llvm_asm)]

extern crate fk_std;

pub const ARCH: &str = "x86_64";

pub mod boot;
pub mod include;
pub mod kernel;
