#![no_std]

#[macro_use] extern crate printk;

use multiboot2::load;

pub unsafe fn multiboot_init(bootinfo_addr: usize) {
    let mut bootinfo = load(bootinfo_addr);

    if bootinfo.is_err() {
        panic!("{:?}", bootinfo.err().unwrap());
    }
}
