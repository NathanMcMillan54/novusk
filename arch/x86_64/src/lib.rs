#![no_std]
#![feature(asm, global_asm)]

extern crate alloc;
extern crate drivers;
extern crate include;
#[macro_use]
extern crate kernel;
extern crate libn;

extern crate ex1;

pub mod boot;
// include
pub mod x86include;
// kernel
pub mod x86kernel;
