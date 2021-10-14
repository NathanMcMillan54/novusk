#![no_std]
#![feature(asm)]

#[macro_use] extern crate printk;

pub mod memory;

use multiboot2::{load, BootInformation};

pub unsafe fn grub_init(address: usize) {
    let bootinfo = load(address);
}
