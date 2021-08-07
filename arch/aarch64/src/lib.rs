#![no_std]
#![feature(asm, global_asm, llvm_asm)]
#![feature(alloc_error_handler, panic_info_message)]
#![feature(core_intrinsics)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate tock_registers;

pub extern crate arm;

pub mod boot;
pub mod kernel;
pub mod mm;
