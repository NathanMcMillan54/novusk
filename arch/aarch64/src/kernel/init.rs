use crate::aarch64_printk;
use init::kmain;
use super::device::{device_init};

pub unsafe fn aarch64_kernel_init() {
    initialize_device();
    kinfo!("Device initialized\n");

    kmain::kernel_init();
    kinfo!("Novusk initialized\n");
}

unsafe fn initialize_device() {
    let (success, name) = device_init();

    if success.is_err() {
        panic!("Failed to initialize device: {}", success.err().unwrap());
    }

    aarch64_printk!("Successfully initialized {}\n", name);
}
