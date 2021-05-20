use crate::drivers::device::DEVICE_INFO;
use crate::kernel::init::aarch64_kernel_init;
use arm_lib::include::asm::{wfe};
use super::{device};

#[no_mangle]
pub unsafe extern "C" fn setup() -> ! {
    device::device_init();
    kinfo!("Device initialized | Machine: {} Manufacture: {} CPU: {}", DEVICE_INFO.board, DEVICE_INFO.manufacture, DEVICE_INFO.cpu);
    if DEVICE_INFO.arch_kernel == true {
        aarch64_kernel_init();
    } else {
        kinfo!("Device doesn't support aarch64 kernel, running machine specific kernel...");
    }
    wfe()
}
