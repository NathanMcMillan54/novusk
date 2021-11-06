#![no_std]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;

#[cfg(any(feature = "hifive", feature = "lofive"))]
pub use sifive as board;

pub mod boot;
pub mod kernel;
pub mod mm;
