#![no_std]
#![feature(panic_info_message)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;
extern crate setup;

#[cfg(feature = "rpi3")]
pub(crate) extern crate rpi;

mod boot;
pub mod include;
pub mod kernel;
pub mod liba64;
pub mod mm;
mod net;

// build.rs generates dif.rs, it should be ignored from git
pub(crate) mod dif;
