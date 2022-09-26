use novuskinc::irq::{irqchip_setup};
use novuskinc::platform::*;
use printk::init::{console_init, error::*};
use crate::SetupReturn;

pub trait ArchKernelSetup {
    fn irq_setup(&self) -> SetupReturn {
        unsafe { irqchip_setup(); }
        return (Ok(()), "Successfully setup IRQ chip");
    }

    fn device_init(&self) -> SetupReturn {
        let mut device = 0;

        unsafe {
            device = device_init();
        }

        if device != 0 {
            return (Err(DEVICE_INIT_ERRORS[device as usize]), "Device init error");
        }

        return (Ok(()), "Device initialized successfully");
    }

    fn input_setup(&self) -> SetupReturn {
        (Ok(()), "Success")
    }

    fn display_init(&self) -> SetupReturn {
        (Ok(()), "Success")
    }

    fn serial_io_init(&self) -> SetupReturn {
        (Ok(()), "Success")
    }

    unsafe fn early_kernel_setup(&self) -> SetupReturn {
        match console_init() {
            SUCCESS => {},
            DRIVER_FAILED => {
                return (Err("Console init error"), "Failed to initialize console driver");
            },
            DRIVER_NOT_FOUND => {
                return (Err("Console init error"), "Console driver not found");
            },
            _ => {}
        }

        (Ok(()), "Successfully setup early main kernel")
    }
}
