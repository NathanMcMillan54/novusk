use super::kernel::{ARM_KERNEL, ArmKernel};
use setup::{SetupReturn, ArchKernelSetup};

impl ArchKernelSetup for ArmKernel {

}

#[no_mangle]
pub unsafe extern "C" fn setup_arch() {
    ARM_KERNEL.setup();
}