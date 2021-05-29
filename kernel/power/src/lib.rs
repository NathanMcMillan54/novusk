#![no_std]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;

mod uefi;
pub use uefi::*;
