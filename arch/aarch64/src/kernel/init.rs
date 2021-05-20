use crate::drivers::device::DEVICE_INFO;
use init::kmain::kernel_init;
use super::{modules, time};

#[no_mangle]
pub unsafe extern "C" fn aarch64_kernel_init() {
    time::time_init();
    kinfo!("Kernel time initialized");

    modules::modules_init();
    kinfo!("aarch64 modules initialized");

    if DEVICE_INFO.main_kernel == true {
        kernel_init();
        kinfo!("Novusk initialized");
    } else if DEVICE_INFO.main_kernel == false {
        kinfo!("Device doesn't support main kernel, starting userspace...");
    }
}