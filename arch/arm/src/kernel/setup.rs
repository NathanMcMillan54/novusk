use super::kernel::{ARM_KERNEL, ArmKernel};
use novuskinc::irq::irqchip_setup;
use novuskinc::platform::device_init;
use setup::{SetupReturn, ArchKernelSetup};


impl ArchKernelSetup for ArmKernel {
    fn irq_setup(&self) -> SetupReturn {
        unsafe { irqchip_setup(); }
        (Ok(()), "IRQS successfully setup")
    }

    fn input_setup(&self) -> SetupReturn {

        (Ok(()), "Success")
    }

    fn display_init(&self) -> SetupReturn {

        (Ok(()), "Success")
    }

    fn serial_io_init(&self) -> SetupReturn {

        (Ok(()), "Success")
    }
}

#[no_mangle]
pub unsafe extern "C" fn setup_arch() {
    ARM_KERNEL.setup();
}
