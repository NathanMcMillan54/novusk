use novuskinc::irq::irqchip_setup;
use kinfo::{InfoDisplay, status::KStatus};
use crate::mm::memory_init;
use super::cpu::{cpu_setup, CPUINFO};
use super::device::{device_init, DEVICE_NAME};

pub unsafe fn setup_arm32_kernel() {
    irqchip_setup();
    printk!("Setup IRQ chip\n");

    cpu_setup();
    kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "CPU setup",
        messages: Some(&["IRQs setup"]),
    });

    setup_device();
    kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "Device initialized",
        messages: None,
    });

    memory_init();
    kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "Memory initialized",
        messages: None,
    });
}

unsafe fn setup_device() {
    let (success, name) = device_init();

    match success {
        Err(e) => {
            panic!("Error while initializing device: {:?}", e);
        },
        _ => DEVICE_NAME = name,
    };
}

