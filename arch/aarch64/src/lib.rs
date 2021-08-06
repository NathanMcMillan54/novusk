#![no_std]
#![feature(asm, global_asm, llvm_asm)]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]

#[macro_use] extern crate alloc;

pub extern crate arm;

pub mod boot;
pub mod kernel;
pub mod mm;
