use novuskinc::serial::early_serial_init;
use crate::librv::libdif::*;
use crate::kernel::platform::RISCV_DEVICE;
use crate::rv_printk;
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

        rv_printk!("Finished boot setup\n");
        rv_printk!("{}\n", io_ret.1);
        rv_printk!("{}\n", ld_mem_ret.1);

        // This for some reason crashes
        /*if io_ret.0.is_err() || ld_mem_ret.0.is_err() {
            panic!("An error occurred during boot setup");
        } else { kinfo!("Boot setup successful\n"); }*/
    }

    pub unsafe fn set_dif(&self) {
        DIF = DIF.parse(DIF_FILE);
        RISCV_DEVICE.dif = DIF.parse(DIF_FILE);
    }
}

impl BootSetup for RiscvBoot {
    fn early_serial_io_init(&self) -> SetupReturn {
        unsafe { early_serial_init(); }

        return (Ok(()), "Early I/O initialized");
    }

    unsafe fn linker_setup(&self) -> SetupReturn {
        extern "C" {
            static mut _sbss: u64;
            static mut _ebss: u64;
        }

        r0::zero_bss(_sbss as *mut u64, _ebss as *mut u64);

        return (Ok(()), "Linker-mem setup");
    }
}
