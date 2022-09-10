use crate::kernel::kernel::{ARM_KERNEL, ArmKernel};
use kinfo::{InfoDisplay, status::KStatus};
use setup::kernel::ArchKernelSetup;
use setup::SetupReturn;

impl ArchKernelSetup for ArmKernel {

}

pub fn setup_arm32_kernel() {
    early_printk!("Setting up kernel...\n");
    unsafe { ARM_KERNEL.setup(); }
}
