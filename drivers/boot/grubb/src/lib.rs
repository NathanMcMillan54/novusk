#![no_std]
#![feature(asm)]
#![feature(ptr_internals)]

#[macro_use] extern crate novuskinc;

use multiboot2::{load, BootInformation};
use nmallocator::{ALLOCATOR, x86_64::*};

pub mod memory;

pub unsafe fn grub_init(address: usize) {
    let bootinfo = load(address);

    memory::early_memory_init(bootinfo.unwrap());

    ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
}
