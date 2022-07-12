use core::ptr::write_volatile;
use cortex_a::registers::CurrentEL;
use tock_registers::interfaces::Readable;
use kinfo::status::KStatus;
use novuskinc::platform::device_init;
use printk::printk_init;
use crate::early_printk;
use super::irq::aarch64_irq_setup;
use setup::{ArchKernelSetup, SetupReturn};
use crate::include::dif::DIF;
use crate::kernel::drivers::DEVICE_DRIVERS;

static mut AARCH64_KERNEL: Aarch64Kernel = Aarch64Kernel::new();

pub struct Aarch64Kernel {
    pub early_kernel: bool,
}

impl Aarch64Kernel {
    pub const fn new() -> Self {
        return Aarch64Kernel {
            early_kernel: true,
        }
    }
    
    pub fn setup(&self) {
        self.test_memory();

        let irq = self.irq_setup();
        let dev = unsafe { self.device_init() };
        let kernel = unsafe { self.early_kernel_setup() };

        if irq.0.is_err() {
            kinfo!(KStatus {
                status: "not ok",
                should_panic: true,
                panic_message: Some(irq.1),
                message1: "Failed to setup Aarch64 IRQs",
                message2: Some("This will cause problems later on in the kernel")
            });
        } else if dev.0.is_err() {
            KStatus {
                status: "not ok",
                should_panic: true,
                panic_message: Some(dev.1),
                message1: "Failed to initialize device",
                message2: None,
            };
        } else if kernel.0.is_err() {
            kinfo!(KStatus {
                status: "not ok",
                should_panic: true,
                panic_message: Some(kernel.1),
                message1: "Failed to setup early kernel",
                message2: Some("This will prevent the main kernel from doing important tasks")
            });
        }

        kinfo!(KStatus {
            status: "ok",
            should_panic: false,
            panic_message: None,
            message1: irq.1,
            message2: None,
        });

        kinfo!(KStatus {
            status: "ok",
            should_panic: false,
            panic_message: None,
            message1: dev.1,
            message2: None,
        });

        kinfo!(KStatus {
            status: "ok",
            should_panic: false,
            panic_message: None,
            message1: kernel.1,
            message2: Some("Preparing Aarch64 for the main kernel...")
        })
    }

    fn test_memory(&self) {
        let mut test_vec = vec![0];
        test_vec.push(1);

        for i in 0..1024 {
            test_vec.push(i);
        }
    }
}

impl ArchKernelSetup for Aarch64Kernel {
    fn irq_setup(&self) -> SetupReturn {
        unsafe { aarch64_irq_setup(); }
        return (Ok(()), "IRQ successfully setup");
    }

    fn device_init(&self) -> SetupReturn {
        unsafe {
            if device_init() == 0 {
                return (Ok(()), "Device successfully initialized");
            } else { return (Err("Device init error"), "Device failed to initialize"); }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn setup_arch() {
    early_printk!("\nStarting Aarch64 kernel setup...\n");

    AARCH64_KERNEL.setup();
    AARCH64_KERNEL.early_kernel = false;
}
