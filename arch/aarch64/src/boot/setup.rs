use crate::drivers::device::DEVICE_INFO;
use crate::kernel::init::aarch64_kernel_init;
use arm_lib::include::asm::{wfe};
use super::{device};

#[no_mangle]
pub unsafe extern "C" fn setup() -> ! {
    printk!("LICENCE:\nMIT License Copyright (c) 2021 Nathan McMillan");
    printk!("Read LICENCE for copyright");

    device::device_init();
    kinfo!("Device initialized");
    if DEVICE_INFO.arch_kernel == true {
        aarch64_kernel_init();
        kinfo!("Aarch64 kernel initialized");
    } else {
        kinfo!("Aarch64 kernel initialized");
        printk!("Ending kernel...");
    }
    wfe()
}
