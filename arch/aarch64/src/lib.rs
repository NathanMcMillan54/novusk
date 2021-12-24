#![no_std]
#![feature(panic_info_message)]
#![feature(asm, global_asm, llvm_asm)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate novuskinc;

pub(crate) use novuskinc::define_syscall;

mod boot;
pub mod include;
pub mod kernel;
pub mod mm;
mod net;
