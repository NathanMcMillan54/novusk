#![no_std]
#![feature(abi_x86_interrupt, const_mut_refs, core_intrinsics)]

#[macro_use] extern crate lazy_static;

#[cfg(feature = "bootloader_rs_0_9_23")]
extern crate pic8259;

pub(crate) extern crate nkernel;
extern crate alloc;

pub mod boot;
pub mod include;
pub mod kernel;
pub mod libx;
pub mod mm;