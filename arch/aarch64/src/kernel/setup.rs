use crate::early_printk;
use super::early_printk::aarch64_setup_early_printk;
use setup::{ArchKernelSetup, SetupReturn};
use crate::include::dif::DIF;
use crate::kernel::early_printk::AARCH64_SERIALIO;

struct Aarch64Setup;

impl Aarch64Setup {
    pub fn new() -> Self {
        return Aarch64Setup;
    }
    
    pub fn setup(&self) {
        self.test_memory();
        let irq = self.irq_setup();

        if irq.0.is_err() {
            panic!("{}", irq.1);
        }

        early_printk!("{}\n", irq.1);

        unsafe { AARCH64_SERIALIO.serial_addr = DIF.uart_addr.unwrap() as *mut u8; }

        early_printk!("Test\n");
    }

    fn test_memory(&self) {
        let mut test_vec = vec![0];
        test_vec.push(1);

        for i in 0..1024 {
            test_vec.push(i);
        }
    }
}

impl ArchKernelSetup for Aarch64Setup {
    fn irq_setup(&self) -> SetupReturn {
        return (Ok(()), "IRQ setup successfully");
    }
}

#[no_mangle]
pub unsafe extern "C" fn aarch64_kernel_setup() {
    aarch64_setup_early_printk();
    early_printk!("Starting Aarch64 kernel...\n");
    early_printk!("Early kernel printing for Aarch64 initialized\n");

    let aarch64_setup = Aarch64Setup::new();

    aarch64_setup.setup();
}
