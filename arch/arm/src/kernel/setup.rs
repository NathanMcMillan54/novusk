use crate::include::dif::dif::DIF;
use super::irq::setup_irqs;
use novuskinc::core::prelude::{early_device_init, early_device_end};
use ::setup::{ArchKernelSetup, SetupReturn};

pub(crate) struct ArmKernel;

impl ArmKernel {
    pub fn new() -> Self {
        return ArmKernel;
    }

    pub fn setup(&self) {
        let irq = self.irq_setup();
        let dev = self.device_init();
        let serial = self.serial_io_init();

        if irq.0.is_err() {
            panic!("{}", irq.1);
        }

        if dev.0.is_err() {
            panic!("{}", irq.1);
        }

        if serial.0.is_err() {
            panic!("{}", serial.1);
        }
    }
}

impl ArchKernelSetup for ArmKernel {
    fn irq_setup(&self) -> SetupReturn {
        unsafe { setup_irqs(); }

        (Ok(()), "")
    }

    fn device_init(&self) -> SetupReturn {
        start_module!(early_device_init, early_device_end);

        (Ok(()), "")
    }

    fn serial_io_init(&self) -> SetupReturn {
        unsafe { super::uart::ARMUART.init(); }

        return (Ok(()), "Set UART address");
    }
}

pub unsafe fn setup_arm_kernel() {
    let arm_kernel = ArmKernel::new();

    arm_kernel.setup();

    #[cfg(target_arch = "aarch64")]
    crate::bits64::arm64_kernel_setup();
}
