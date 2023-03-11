use crate::kernel::irq::aarch64_irq_setup;
use super::kernel::{AARCH64_KERNEL, Aarch64Kernel};

#[no_mangle]
pub unsafe extern "C" fn setup_arch() {
    AARCH64_KERNEL.setup();
}
