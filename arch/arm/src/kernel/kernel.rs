use novuskinc::kernel::{arch_prepare_init, kernel_init, setup_arch};
use setup::ArchKernelSetup;
use crate::include::dif::DIF;

pub struct ArmKernel {
    pub dev_kernel: Option<extern "C" fn()>
}

impl ArmKernel {
    pub fn setup(&mut self) {
        let dev = self.device_init();
        let irq = self.irq_setup();

        if dev.0.is_err() {
            panic!("{}", dev.1);
        } else if irq.0.is_err() {
            panic!("{}", irq.1);
        }

        if dev.0.is_ok() {
            printk!("{}", dev.1);
        } else if irq.0.is_ok() {
            printk!("{}", irq.1);
        }
    }
}

pub static mut ARM_KERNEL: ArmKernel = ArmKernel {
    dev_kernel: None,
};

#[no_mangle]
pub unsafe extern "C" fn start_kernel() {
    setup_arch();
    arch_prepare_init();

    let start_init = DIF.get("StartInit").1.parse::<bool>().unwrap_or(false);
    let dev_kernel = DIF.get("DeviceSpecificKernel").1.parse::<bool>().unwrap_or(false);

    if start_init {
        kernel_init();
    } else {
        printk!("kernel_init doesn't need to be started\n");
    }

    if dev_kernel {
        (ARM_KERNEL.dev_kernel.unwrap_or(
            panic!("Couldn't find device specific kernel")
        ))();
    }
}
