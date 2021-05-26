use crate::drivers::device::DEVICE_INFO;
use super::{modules, time};

#[no_mangle]
pub unsafe extern "C" fn aarch64_kernel_init() {
    time::time_init();

    modules::modules_init();

    if DEVICE_INFO.main_kernel == true {
    } else if DEVICE_INFO.main_kernel == false {
    }
}
