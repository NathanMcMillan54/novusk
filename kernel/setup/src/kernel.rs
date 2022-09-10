use novuskinc::irq::{irqchip_setup};
use novuskinc::platform::*;
use printk::init::console_init;
use crate::SetupReturn;

pub trait ArchKernelSetup {
    fn irq_setup(&self) -> SetupReturn {
        unsafe { irqchip_setup(); }
        return (Ok(()), "Successfully setup IRQ chip");
    }

    fn device_init(&self) -> SetupReturn {
        let mut early_device= 0;
        let mut device = 0;

        unsafe {
            early_device = early_device_init();
            device = device_init();
        }

        if early_device != 0 {
            return (Err("Device init error"), DEVICE_INIT_ERRORS[early_device as usize]);
        } else if device != 0 {
            return (Err("Device init error"), DEVICE_INIT_ERRORS[device as usize]);
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
        console_init();

        (Ok(()), "Successfully setup early main kernel")
    }
}
