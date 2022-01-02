use crate::types::SetupReturn;

pub trait BootSetup {
    fn early_io_init(&self) -> SetupReturn {
        return (Ok(()), "Early I/O successfully initialized");
    }

    unsafe fn linker_setup(&self) -> SetupReturn {
        return (Ok(()), "Successfully setup linker memory");
    }

    fn disable_dwt(&self) -> SetupReturn {
        return (Ok(()), "DWT successfully initialized");
    }

    fn cpuid_init(&self) -> SetupReturn {
        return (Ok(()), "Successfully got and set CPU info");
    }
}

