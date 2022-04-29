#![no_std]
#![feature(panic_info_message)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate novuskinc;

#[cfg(feature = "rpi3")]
pub(crate) extern crate rpi;

pub(crate) use novuskinc::define_syscall;

mod boot;
pub mod include;
pub mod kernel;
pub mod mm;
mod net;
