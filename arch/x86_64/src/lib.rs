#![no_std]
#![feature(abi_x86_interrupt)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate novuskinc;

pub(crate) extern crate nkernel;

pub mod boot;
pub mod kernel;
pub mod libx;
pub mod mm;