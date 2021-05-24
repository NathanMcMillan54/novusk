#![no_std]

#[macro_use] extern crate kinfo;

extern "C" {
    fn initramfs() -> bool;
    fn initramfs_main();
    fn kernel_initramfs_main();
}

pub unsafe fn init_ramfs() {
    if initramfs() {
        initramfs_main();
    } else {
        // TODO: Write documnetation on initramfs
        kinfo!("Running kernel initramfs, read Documentation/initramfs/main.md");
        kernel_initramfs_main();
    }
}
