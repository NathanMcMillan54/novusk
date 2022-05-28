use crate::include::dif::DIF;
use crate::kernel::uart::is_init;
use crate::mm::bss::*;
use dif::Dif;
use novuskinc::serial::early_serial_init;
use setup::{BootSetup, SetupReturn};

pub struct Aarch64Boot;

impl Aarch64Boot {
    pub fn new() -> Self {
        return Aarch64Boot;
    }

    pub fn setup(&self) {
        let dif = unsafe { self.dif_init() };
        let ld = unsafe { self.linker_setup() };
        let early_io = self.early_serial_io_init();

        if dif.0.is_err() {
            panic!("{}", dif.1)
        } else if ld.0.is_err() {
            panic!("{}", ld.1);
        } else if early_io.0.is_err() {
            panic!("{}", early_io.1);
        }

        crate::early_printk!("{}\n", dif.1);
        crate::early_printk!("{}\n", ld.1);
        crate::early_printk!("{}\n", early_io.1);
    }

    pub unsafe fn dif_init(&self) -> SetupReturn {
        extern "C" {
            static mut DIF_FILE: &'static [(&'static str, &'static str); 11];
        }

        DIF.set_and_parse(DIF_FILE);

        if DIF == Dif::empty() {
            return (Err("Failed to initialize DIF"), "DIF is empty");
        } else { return (Ok(()), "Successfully set DIF"); }
    }
}

impl BootSetup for Aarch64Boot {
    fn early_serial_io_init(&self) -> SetupReturn {
        unsafe {
            early_serial_init();

            if is_init() {
                return (Ok(()), "Early serial I/O successfully initialized");
            } else { return (Err("Failed to initialize serial I/O"), "Early serial I/O could not be initialized"); }
        }
    }

    unsafe fn linker_setup(&self) -> SetupReturn {
        extern "C" {
            static mut __bss_start: u32;
            static mut __bss_end: u32;
        }

        zero_bss(&mut __bss_start, &mut __bss_end);

        if is_zeroed(__bss_end) {
            return (Ok(()), "BSS sections successfully cleared");
        } else { return (Err("Failed to initialize linker memory"), "Failed to clear BSS"); }
    }
}
