use alloc::string::String;
use crate::init::KERNEL;

pub fn initramfs_type() -> String {
    let initramfs = KERNEL.lock().kernel_configs().get("USERSPACE", "KERNELINITRAMFS");

    return initramfs;
}


pub(crate) fn start_kernel_initramfs() {
    super::noinitramfs::no_initramfs()
}

pub(crate) unsafe fn start_custom_initramfs() {
    extern "C" {
        fn initramfs_main();
    }

    initramfs_main();
}
