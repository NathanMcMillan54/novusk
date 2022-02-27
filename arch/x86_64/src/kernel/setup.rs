use core::ptr::write_volatile;
use kinfo::status::KStatus;
use super::irq::start_irq_setup;
use novuskinc::core::prelude::*;
use printk::PRINTK;
use setup::{ArchKernelSetup, SetupReturn};
use x86_64::instructions::bochs_breakpoint;
use crate::early_printk;

struct X86_64Kernel;

impl X86_64Kernel {
    pub fn new() -> Self {
        return X86_64Kernel;
    }
    
    pub fn setup(&self) {
        let irq = self.irq_setup();
        let display = self.display_init();

        if irq.0.is_err() {
            kinfo!(KStatus {
                status: "not ok",
                should_panic: true,
                panic_message: Some("Failed setup IRQs"),
                message1: irq.1,
                message2: Some("IRQs won't be initialized later which will cause huge problems")
            });
        } else if display.0.is_err() {
            kinfo!(KStatus {
                status: "not ok",
                should_panic: false,
                panic_message: None,
                message1: display.1,
                message2: Some("This won't prevent the kernel from running"),
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
            message1: display.1,
            message2: None,
        });
    }
}


impl ArchKernelSetup for X86_64Kernel {
    fn irq_setup(&self) -> SetupReturn {
        unsafe { start_irq_setup(); }

        return (Ok(()), "IRQs setup");
    }

    fn display_init(&self) -> SetupReturn {
        start_module!(core_display_init, core_display_end);
        (Ok(()), "Display initialized")
    }

    fn serial_io_init(&self) -> SetupReturn {
        (Ok(()), "Serial setup")
    }
}

pub unsafe fn setup_x86_64() {
    let kernel = X86_64Kernel::new();
    kernel.setup();
}

