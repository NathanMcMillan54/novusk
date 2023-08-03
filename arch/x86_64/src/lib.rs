#![no_std]
#![feature(abi_x86_interrupt, const_mut_refs)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate novuskinc;

#[cfg(feature = "bootloader_rs_0_9_23")]
extern crate pic8259;

pub(crate) extern crate nkernel;

pub mod boot;
pub mod kernel;
pub mod libx;
pub mod mm;