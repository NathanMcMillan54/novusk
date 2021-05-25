#![no_std]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;


extern crate novusk;

pub mod kernel_initramfs;
pub mod kmain;
pub mod version;
