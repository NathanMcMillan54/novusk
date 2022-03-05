use crate::SetupReturn;

pub trait ArchKernelSetup {
    fn irq_setup(&self) -> SetupReturn {
        (Ok(()), "Success")
    }

    fn device_init(&self) -> SetupReturn {
        (Ok(()), "Success")
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
            fn set_kernel_printer();
        }

        //set_kernel_printer();

        (Ok(()), "Successfully setup early main kernel")
    }
}
