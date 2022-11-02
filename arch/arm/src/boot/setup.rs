use core::fmt::Write;
use dif::DifFieldNames;
use novuskinc::serial::early_serial_init;
use setup::{BootSetup, SetupReturn};
use crate::liba32::libdif::DIF;
use crate::mm::linker::clear_bss_sections;
use hio::io::get_hio;

pub struct ArmBootSetup;

impl ArmBootSetup {
    pub fn new() -> Self {
        return ArmBootSetup;
    }

    pub unsafe fn setup(&self) {
        let early_mem = unsafe { self.linker_setup() };
        let early_serial = if unsafe { DIF.get(DifFieldNames::EnableSerial).parse::<bool>().unwrap_or(false) } {
            self.early_serial_io_init()
        } else { (Ok(()), "Serial driver doesn't need to be initialized") };
        let early_dev = self.early_device_init();


    }
}

impl BootSetup for ArmBootSetup {
    unsafe fn linker_setup(&self) -> SetupReturn {
        // #[cfg(feature = "cortex_a")]
        clear_bss_sections();

        (Ok(()), "Successfully setup linker memory")
    }
}
