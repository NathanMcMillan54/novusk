#![no_std]
#![feature(asm, global_asm)]
#![feature(ptr_internals)]

#[macro_use]
extern crate kernel;

extern crate spin;
extern crate volatile;

pub mod boot;
pub mod drivers;
pub mod include;
// kernel
pub mod akernel;
pub mod x86lib;
