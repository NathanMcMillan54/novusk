#![no_std]
#![feature(global_asm, llvm_asm)]

#[macro_use] extern crate log;
#[macro_use] extern crate uefi;
#[macro_use] extern crate uefi_macros;
#[macro_use] extern crate uefi_services;

#[cfg(target_arch = "x86")]
extern crate i686;

pub(crate) mod boot;
pub mod drivers;
pub mod include;
pub mod kernel;
