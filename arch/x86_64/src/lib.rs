#![no_std]
#![feature(asm, global_asm)]

extern crate drivers;
#[macro_use]
extern crate kernel;
extern crate raw_cpuid;

pub mod boot;
pub mod include;
// kernel
pub mod akernel;
