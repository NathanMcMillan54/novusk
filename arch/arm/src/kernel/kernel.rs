use novuskinc::kernel::{arch_prepare_init, kernel_init, setup_arch};
use crate::include::dif::DIF;

pub struct ArmKernel {
    pub dev_kernel: Option<extern "C" fn()>
}

impl ArmKernel {
    pub fn setup(&mut self) {

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
        // printk!("Kernel init doesn't need to be started\n");
    }

    if dev_kernel {
        (ARM_KERNEL.dev_kernel.unwrap_or(
            panic!("Couldn't find device specific kernel")
        ))();
    }
}
