use crate::drivers::device::DEVICE_INFO;
use init::kmain::kernel_init;
use super::{modules, time};

#[no_mangle]
pub unsafe extern "C" fn aarch64_kernel_init() {
    time::time_init();
    kinfo!("Kernel time initialized");

    modules::modules_init();
    kinfo!("Aarch64 kernel modules initialized");

    if DEVICE_INFO.main_kernel == true {
        printk!("Starting kernel init...");
        kernel_init();
    } else if DEVICE_INFO.main_kernel == false {
        kinfo!("Aarch64 kernel initialized");
    }
}
