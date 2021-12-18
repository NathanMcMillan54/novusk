#![no_std]

#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;

pub mod modules;
pub use modules::KernelModules;
pub mod start;
pub use start::*;

extern crate ex1;
extern crate fscheck;

#[cfg(target_arch = "x86_64")]
extern crate vgag;

#[cfg(target_arch = "aarch64")]
extern crate armfb;