use crate::arm32_printk;
use crate::mm::memory_init;
use super::cpu::cpu_setup;
use super::device::{device_init, DEVICE_NAME};

pub unsafe fn setup_arm32_kernel() {
    cpu_setup();
    kinfo!("Setup CPU\n");
    arm32_printk!("    Common IRQs setup\n");

    setup_device();
    kinfo!("Device initialized\n");

    memory_init();
    kinfo!("Memory initialized\n");
}

unsafe fn setup_device() {
    let (success, name) = device_init();

    match success {
        Err(e) => {
            panic!("Error while initializing device: {:?}", e);
            kinfo::status::set_status("not ok");
        },
        _ => DEVICE_NAME = name,
    };
}

