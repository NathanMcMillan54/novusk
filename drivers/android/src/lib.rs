#![no_std]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;

pub mod color;
pub mod kernel;

#[cfg(feature = "box")]
pub mod android_box;
