use setup::{BootSetup, SetupReturn};

pub struct RiscvBoot;

impl RiscvBoot {
    pub fn new() -> Self {
        return RiscvBoot;
    }

    pub fn setup(&self) {
        let io_ret = self.early_io_init();
        let ld_mem_ret = unsafe { self.linker_setup() };
    }
}

impl BootSetup for RiscvBoot{
    fn early_io_init(&self) -> SetupReturn {
        #[cfg(any(feature = "hifive", feature = "lofive"))]
        sifive::SiFiveIo::new().sifive_io_init();

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
