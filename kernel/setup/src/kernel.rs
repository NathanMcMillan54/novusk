use novuskinc::platform::device_init;
use printk::printk_init;
use crate::SetupReturn;

pub trait ArchKernelSetup {
    fn irq_setup(&self) -> SetupReturn {
        (Ok(()), "Success")
    }

    fn device_init(&self) -> SetupReturn {
        unsafe {
            if device_init() == 0 {
                (Ok(()), "Successfully initialized device")
            } else { (Err("Device init error"), "Failed to initialize device") }
        }
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
        printk_init("Graphics Driver");

        (Ok(()), "Successfully setup early main kernel")
    }
}
