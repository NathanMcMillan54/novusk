use crate::arm32_printk;
use crate::mm::memory_init;
use super::device::{device_init, DEVICE_NAME};

pub unsafe fn setup_arm32_kernel() {
    setup_device();
    arm32_printk!("Device initialized\n");

    memory_init();
    arm32_printk!("Memory initialized\n");
}

unsafe fn setup_device() {
    let (success, name) = device_init();

    match success {
        Err(e) => panic!("Error while initializing device: {:?}", e),
        _ => DEVICE_NAME = name,
    };
}

