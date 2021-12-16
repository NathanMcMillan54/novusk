#![no_std]
#![feature(panic_info_message)]
#![feature(asm, global_asm, llvm_asm)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate novuskinc;

mod boot;
pub(crate) mod kernel;
pub(crate) mod mm;
mod net;
