use novuskinc::platform::device_init;
use crate::SetupReturn;

pub trait ArchKernelSetup {
    unsafe fn device_init(&self) -> SetupReturn {
        let init = device_init();

        return if init == 0 {
            (Ok(()), "Device successfully initialized")
        } else { (Err("Device init error"), "Device failed to initialize") }
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
