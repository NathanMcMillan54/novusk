use core::fmt::Write;
use dif::DifFieldNames;
use novuskinc::serial::early_serial_init;
use setup::{BootSetup, SetupReturn};
use crate::liba32::libdif::DIF;
use crate::mm::linker::clear_bss_sections;
use hio::io::get_hio;
use kinfo::{status::KStatus, InfoDisplay};

pub struct ArmBootSetup;

impl ArmBootSetup {
    pub fn new() -> Self {
        return ArmBootSetup;
    }

    pub unsafe fn setup(&self) {
        let early_mem = unsafe { self.linker_setup() };
        let early_dev = self.early_device_init();
        let early_serial = if unsafe { DIF.get(DifFieldNames::EnableSerial).parse::<bool>().unwrap_or(false) } {
            self.early_serial_io_init()
        } else { (Ok(()), "Serial driver doesn't need to be initialized") };


        if early_mem.0.is_err() {
            kinfo!(KStatus {
                status: "not ok",
                should_panic: true,
                panic_message: None,
                main_message: early_mem.1,
                messages: None,
            });
        } else if early_serial.0.is_err() {
            kinfo!(KStatus {
                status: "not ok",
                should_panic: false,
                panic_message: None,
                main_message: early_serial.1,
                messages: Some(&["This is just an inconvenience"]),
            });
        } else if early_dev.0.is_err() {
            kinfo!(KStatus {
                status: "not ok",
                should_panic: true,
                panic_message: Some("This will cause error latter"),
                main_message: early_dev.1,
                messages: None,
            })
        }

        early_printk!("Boot setup finished\n");

        kinfo!(KStatus {
            status: "ok",
            should_panic: false,
            panic_message: None,
            main_message: early_mem.1,
            messages: None,
        });

        kinfo!(KStatus {
            status: "ok",
            should_panic: false,
            panic_message: None,
            main_message: early_serial.1,
            messages: None,
        });

        kinfo!(KStatus {
            status: "ok",
            should_panic: false,
            panic_message: None,
            main_message: early_dev.1,
            messages: Some(&["Some device specific drivers have been set"]),
        });
    }
}

impl BootSetup for ArmBootSetup {
    unsafe fn linker_setup(&self) -> SetupReturn {
        // The cortex-m-rt crate should do this
        #[cfg(feature = "cortex_a")]
        clear_bss_sections();

        (Ok(()), "Successfully setup linker memory")
    }
}
