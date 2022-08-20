use dif::{DifFieldNames};
use novuskinc::serial::early_serial_init;
use crate::librv::libdif::*;
use crate::kernel::platform::RISCV_DEVICE;
use setup::{BootSetup, SetupReturn};

pub struct RiscvBoot;

impl RiscvBoot {
    pub fn new() -> Self {
        return RiscvBoot;
    }

    pub fn setup(&self) {
        unsafe { self.set_dif(); }
        let io_ret = self.early_serial_io_init();
        let ld_mem_ret = unsafe { self.linker_setup() };

        if early_printk!("Early kernel printing is working\n").is_err() {
            panic!("{}", io_ret.1);
        }

        if ld_mem_ret.0.is_err() {
            panic!("{}", ld_mem_ret.1);
        }
    }

    pub unsafe fn set_dif(&self) {
        DIF = DIF.parse(DIF_FILE);
        RISCV_DEVICE.dif = DIF.parse(DIF_FILE);
    }
}

impl BootSetup for RiscvBoot {
    fn early_serial_io_init(&self) -> SetupReturn {
        unsafe {
            if RISCV_DEVICE.dif.get(DifFieldNames::EnableSerial).parse::<bool>().unwrap_or(false) {
                if early_serial_init() != 0 {
                    return (Ok(()), "Early I/O initialized");
                } else { return (Err("Failed to initialize serial I/O"), "Failed to initialize serial I/O"); }
            }
        }

        (Ok(()), "Device doesn't need to initialize serial I/O")
    }

    unsafe fn linker_setup(&self) -> SetupReturn {
        extern "C" {
            static mut _sbss: u64;
            static mut _ebss: u64;
        }

        r0::zero_bss(_sbss as *mut u64, _ebss as *mut u64);

        return (Ok(()), "BSS sections cleared");
    }
}
