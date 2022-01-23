use ::setup::{ArchKernelSetup, SetupReturn};
use novuskinc::core::prelude::{early_device_init, early_device_end};

struct ArmKernel;

impl ArmKernel {
    pub fn new() -> Self {
        return ArmKernel;
    }

    pub fn setup(&self) {
        let irq = self.irq_setup();
        let dev = self.device_init();

        if irq.0.is_err() {
            panic!("{}", irq.1);
        }

        if dev.0.is_err() {
            panic!("{}", irq.1);
        }
    }
}

impl ArchKernelSetup for ArmKernel {
    fn irq_setup(&self) -> SetupReturn {
        (Ok(()), "")
    }

    fn device_init(&self) -> SetupReturn {
        start_module!(early_device_init, early_device_end);
        (Ok(()), "")
    }
}

pub unsafe fn setup_arm_kernel() {
    let arm_kernel = ArmKernel::new();

    arm_kernel.setup();

    #[cfg(target_arch = "aarch64")]
    crate::bits64::arm64_kernel_setup();
}
