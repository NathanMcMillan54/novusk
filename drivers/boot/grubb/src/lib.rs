#![no_std]
#![feature(asm)]
#![feature(ptr_internals)]

#[macro_use] extern crate novuskinc;

pub mod memory;

use multiboot2::{load, BootInformation};

pub unsafe fn grub_init(address: usize) {
    let bootinfo = load(address);

    memory::early_memory_init(bootinfo.unwrap());
}
