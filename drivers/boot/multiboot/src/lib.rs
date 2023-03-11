#![no_std]


use multiboot2::load;

pub unsafe fn multiboot_init(bootinfo_addr: usize) {
    let mut bootinfo = load(bootinfo_addr);
}
