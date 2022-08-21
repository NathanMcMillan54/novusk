use armfb::{armfb_end, armfb_init};
use init::kmain;
use kinfo::{InfoDisplay, status::KStatus};
use super::device::{device_init};
use setup::after_kernel_setup;
use crate::include::sys::setup::syscalls_init;

pub unsafe fn aarch64_kernel_init() {
    initialize_device();
    kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "Device initialized",
        messages: None,
    });

    /*syscalls_init();
    kinfo!("System calls initialized\n");*/

    kmain::kernel_init();
    kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "Novusk initialized",
        messages: None,
    });

    extern "C" {
        fn kernel_main();
    }

    early_printk!("Starting main...\n");
    after_kernel_setup();
    kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "After kernel initialized",
        messages: None,
    });

    kernel_main();
}

unsafe fn initialize_device() {
    let (success, name) = device_init();

    if success.is_err() {
        panic!("Failed to initialize device: {}", success.err().unwrap());
    }

    early_printk!("Successfully initialized {}\n", name);
}
