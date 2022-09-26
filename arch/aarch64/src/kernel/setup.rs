use setup::kernel::ArchKernelSetup;
use super::kernel::{AARCH64_KERNEL, Aarch64Kernel};

impl ArchKernelSetup for Aarch64Kernel {

}

#[no_mangle]
pub unsafe extern "C" fn setup_arch() {
    AARCH64_KERNEL.setup();
}
