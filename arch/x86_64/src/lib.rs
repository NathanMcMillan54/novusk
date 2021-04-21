#![no_std]
#![feature(asm, global_asm)]

extern crate drivers;
extern crate include;
#[macro_use]
extern crate kernel;
extern crate raw_cpuid;

extern crate ex1;

pub mod boot;
// include
pub mod ainclude;
// kernel
pub mod akernel;
