use armfb::{armfb_end, armfb_init};
use crate::aarch64_printk;
use init::kmain;
use super::device::{device_init};
use setup::after_kernel_setup;
use crate::include::sys::setup::syscalls_init;

pub unsafe fn aarch64_kernel_init() {
    initialize_device();
    kinfo!("Device initialized\n");

    syscalls_init();
    kinfo!("System calls initialized\n");

    kmain::kernel_init();
    kinfo!("Novusk initialized\n");

    extern "C" {
        fn kernel_main();
    }

    aarch64_printk!("Starting main...\n");
    after_kernel_setup();
    kinfo!("After kernel initialized\n");

    kernel_main();
}

unsafe fn initialize_device() {
    let (success, name) = device_init();

    if success.is_err() {
        panic!("Failed to initialize device: {}", success.err().unwrap());
    }

    aarch64_printk!("Successfully initialized {}\n", name);
}
