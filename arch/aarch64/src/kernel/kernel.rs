use kinfo::{InfoDisplay, status::KStatus};
use printk::init::printk_init;
use setup::kernel::ArchKernelSetup;

#[no_mangle]
extern "C" fn irqchip_setup() {}

pub(crate) struct Aarch64Kernel {
    pub early: bool,
}

impl Aarch64Kernel {
    pub fn new() -> Self {
        return Aarch64Kernel {
            early: true,
        };
    }

    pub fn setup(&self) {
        let irq = self.irq_setup();
        let device_init = self.device_init();
        let early_kernel = unsafe { self.early_kernel_setup() };

        if irq.0.is_err() {
            kinfo!(KStatus {
                status: "not ok",
                should_panic: true,
                panic_message: None,
                main_message: irq.1,
                messages: Some(&["This will cause errors later"]),
            });
        } else if device_init.0.is_err() {
            kinfo!(KStatus {
                status: "not ok",
                should_panic: false,
                panic_message: None,
                main_message: device_init.0.err().unwrap(),
                messages: Some(&["This will cause problems later"]),
            });
        } else if early_kernel.0.is_err() {
            kinfo!(KStatus {
                status: "not ok",
                should_panic: true,
                panic_message: Some(early_kernel.1),
                main_message: early_kernel.0.err().unwrap(),
                messages: Some(&["This will cause problems later"]),
            });
        }
    }
}

pub(crate) static mut AARCH64_KERNEL: Aarch64Kernel = Aarch64Kernel {
    early: true,
};
