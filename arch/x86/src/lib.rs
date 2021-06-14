#![no_std]
#![feature(asm, global_asm)]

#[macro_use] extern crate libefi;

pub mod boot;
pub mod kernel;
