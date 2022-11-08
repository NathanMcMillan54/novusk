use super::irq::test_exception;
use super::kernel::{ARM_KERNEL, ArmKernel};
use kinfo::{InfoDisplay, status::KStatus};
use novuskinc::irq::irqchip_setup;
use setup::kernel::ArchKernelSetup;
use setup::SetupReturn;

impl ArchKernelSetup for ArmKernel {
    unsafe fn irq_setup(&self) -> SetupReturn {
        irqchip_setup();

        if test_exception() != 0 {
            return (Err("IRQ setup error"), "Failed to setup IRQ chip, CPU exceptions aren't working");
        } else { return (Ok(()), "Successfully setup IRQ chip"); }
    }
}

pub fn setup_arm32_kernel() {
    unsafe { ARM_KERNEL.setup(); }
}
