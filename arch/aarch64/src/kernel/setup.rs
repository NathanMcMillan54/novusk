use core::ptr::write_volatile;
use printk::printk_init;
use crate::early_printk;
use super::early_printk::aarch64_setup_early_printk;
use setup::{ArchKernelSetup, SetupReturn};
use crate::include::dif::DIF;
use crate::kernel::early_printk::AARCH64_SERIALIO;

struct Aarch64Kernel;

impl Aarch64Kernel {
    pub fn new() -> Self {
        return Aarch64Kernel;
    }
    
    pub fn setup(&self) {
        self.test_memory();
        let kernel = unsafe { self.early_kernel_setup() };
        let irq = self.irq_setup();

        if irq.0.is_err() {
            panic!("{}", irq.1);
        } else if kernel.0.is_err() {
            panic!("{}", kernel.1);
        }

        early_printk!("{}\n", irq.1);
        early_printk!("{}\n", kernel.1);
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
        return (Ok(()), "IRQ setup successfully");
    }

    unsafe fn early_kernel_setup(&self) -> SetupReturn {
        printk_init("Console Driver");
        return (Ok(()), "Early main kernel setup successfully");
    }
}

#[no_mangle]
pub unsafe extern "C" fn aarch64_kernel_setup() {
    aarch64_setup_early_printk();
    early_printk!("Starting Aarch64 kernel...\n");
    early_printk!("Early kernel printing for Aarch64 initialized\n");
    early_printk!("\nSetting up kernel...\n");

    let aarch64_setup = Aarch64Kernel::new();

    aarch64_setup.setup();
}
