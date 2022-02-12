use super::irq::start_irq_setup;
use novuskinc::core::prelude::*;
use setup::{ArchKernelSetup, SetupReturn};

struct X86_64Kernel;

impl X86_64Kernel {
    pub fn new() -> Self {
        return X86_64Kernel;
    }
    
    pub fn setup(&self) {
        self.irq_setup();
    }
}

impl ArchKernelSetup for X86_64Kernel {
    fn irq_setup(&self) -> SetupReturn {
        unsafe { start_irq_setup(); }
        return (Ok(()), "IRQs setup");
    }

    fn serial_io_init(&self) -> SetupReturn {
        (Ok(()), "Serial setup")
    }

    fn display_init(&self) -> SetupReturn {
        start_module!(core_display_init, core_display_end);
        (Ok(()), "Display initialized")
    }
}

pub unsafe fn setup_x86_64() {
    let kernel = X86_64Kernel::new();
    kernel.setup();
}