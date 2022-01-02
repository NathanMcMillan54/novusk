use super::irq::irq_init;
use crate::rv_printk;
use crate::mm::memory_init;
use setup::arch::ArchKernelSetup;
use setup::SetupReturn;

struct RiscvKernel;

impl RiscvKernel {
    pub fn new() -> Self {
        return RiscvKernel;
    }

    pub fn setup(&self) {
        let irq_ret = self.irq_init();
        let mem_ret = self.memory_setup();

        rv_printk!("Finished RISCV kernel setup\n");
        rv_printk!("{}\n{}\n", irq_ret.1, mem_ret.1);
    }
}

impl ArchKernelSetup for RiscvKernel {
    fn memory_setup(&self) -> SetupReturn {
        if memory_init().is_ok() {
            return (Ok(()), "Successfully setup memory");
        } else { return (Err("An error ouccourd while initializing memory"), "Failed to setup memory"); }
    }

    fn irq_init(&self) -> SetupReturn {
        unsafe { irq_init(); }

        return (Ok(()), "Enabled IRQs");
    }
}

pub fn setup_riscv_kernel() {
    let kernel_setup = RiscvKernel::new();

    kernel_setup.setup();
    kinfo!("Setup finish\n");

    rv_printk!("Starting after kernel...\n");
    // TODO: Setup after kernel if init can't be called
}
