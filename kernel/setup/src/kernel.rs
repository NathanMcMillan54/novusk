use dif::DifFieldNames;
// use novuskinc::console::{console_init, printk_init};
use novuskinc::irq::{irqchip_setup};
use novuskinc::platform::*;
use printk::init::error::*;
use crate::SetupReturn;
use crate::libdif::DIF;

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
        extern "C" {
            pub fn console_init() -> u8;
            pub fn printk_init() -> u8;
        }

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

        match printk::printk_init(DIF.get(DifFieldNames::PrintingMethod)) {
            SUCCESS => {}
            DRIVER_FAILED => {
                return (Err("Printk init error"), "");
            },
            DRIVER_NOT_FOUND => {
                return (Err("Printk init error"), "Console driver not found");
            },
            _ => {}
        }

        (Ok(()), "Successfully setup early main kernel")
    }
}
