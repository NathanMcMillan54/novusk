use crate::SetupReturn;

pub trait ArchKernelSetup {
    unsafe fn device_init(&self) -> SetupReturn {
        extern "C" {
            fn device_init() -> SetupReturn;
        }

        return device_init();
    }

    fn memory_setup(&self) -> SetupReturn {
        return (Ok(()), "Memory setup successfully");
    }

    fn irq_init(&self) -> SetupReturn {
        return (Ok(()), "IRQ initialized successfully");
    }

    unsafe fn sys_setup(&self) -> SetupReturn {
        return (Ok(()), "System functions setup successfully");
    }
}
