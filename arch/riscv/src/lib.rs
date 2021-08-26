#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler, asm, global_asm)]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate riscv_rt;

pub mod boot;
pub mod include;
pub mod kernel;

#[cfg(not(feature = "no-mem"))]
pub mod mm;
