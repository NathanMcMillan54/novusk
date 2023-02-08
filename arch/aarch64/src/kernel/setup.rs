use setup::kernel::ArchKernelSetup;
use setup::SetupReturn;
use crate::kernel::irq::aarch64_irq_setup;
use super::kernel::{AARCH64_KERNEL, Aarch64Kernel};

impl ArchKernelSetup for Aarch64Kernel {
    unsafe fn irq_setup(&self) -> SetupReturn {
        aarch64_irq_setup();
        (Ok(()), "IRQ setup")
    }
}

#[no_mangle]
pub unsafe extern "C" fn setup_arch() {
    AARCH64_KERNEL.setup();
}
