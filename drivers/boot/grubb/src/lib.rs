#![no_std]
#![feature(asm)]
#![feature(ptr_internals)]
#![feature(allocator_api)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate novuskinc;

use multiboot2::{load, BootInformation};
use nmallocator::{ALLOCATOR, x86_64::*};

pub mod memory;

pub unsafe fn grub_init(address: usize) {
    let bootinfo = load(address);

    if !bootinfo.is_ok() {
        panic!("Cannot unwrap bootinfo");
    }

    memory::early_memory_init(&bootinfo.unwrap());

    // ALLOCATOR.lock().init(HEAP_START, HEAP_START + HEAP_SIZE);
}
