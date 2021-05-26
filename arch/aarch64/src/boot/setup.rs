use crate::drivers::device::DEVICE_INFO;
use crate::kernel::init::aarch64_kernel_init;
use arm_lib::include::asm::{wfe};
use super::{device};

#[no_mangle]
pub unsafe extern "C" fn setup() -> ! {
    device::device_init();
    if DEVICE_INFO.arch_kernel == true {
        aarch64_kernel_init();
    } else {
    }
    wfe()
}
