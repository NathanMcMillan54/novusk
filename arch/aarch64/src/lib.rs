#![no_std]
#![feature(global_asm)]

extern crate arm;

pub const ARCH: &'static str = "aarch64";

pub mod boot;
pub mod kernel;
