use dif::DifFieldNames;
use kinfo::*;
use novuskinc::kernel::kernel_init;
use novuskinc::platform::*;
use kinfo::status::KStatus;
use setup::kernel::ArchKernelSetup;
use crate::liba32::libdif::DIF;

#[no_mangle]
pub unsafe extern "C" fn start_kernel() {
    super::setup::setup_arm32_kernel();
    super::arm_init::arm_kernel_init();

    if DIF.get(DifFieldNames::StartInit).parse::<bool>().unwrap_or(false) {
        printk!("Architecture setup is finished, starting kernel_init...\n");
        kernel_init();
    }
}

pub static mut ARM_KERNEL: ArmKernel = ArmKernel::new();

pub struct ArmKernel {
    pub early: bool,
}

impl ArmKernel {
    pub const fn new() -> Self {
        return ArmKernel {
            early: true,
        }
    }

    pub fn setup(&self) {
        let irq = self.irq_setup();
        let device = self.device_init();

        if device.0.is_err() {
            kinfo!(KStatus {
                status: "not ok",
                should_panic: true,
                panic_message: Some(device.1),
                main_message: "Failed to initialize device",
                messages: Some(&["This would cause errors later"]),
            });
        } else if irq.0.is_err() {
            kinfo!(KStatus {
                status: "ok",
                should_panic: true,
                panic_message: Some(irq.1),
                main_message: "Failed to setup IRQs",
                messages: None,
            });
        }

        kinfo!(KStatus {
            status: "ok",
            should_panic: false,
            panic_message: None,
            main_message: device.1,
            messages: Some(&[
                "Added device specific drivers",
                "Initialized some device specific drivers",
            ]),
        });

        kinfo!(KStatus {
            status: "ok",
            should_panic: false,
            panic_message: None,
            main_message: irq.1,
            messages: None,
        });
    }
}
