use crate::kernel::cpu::info::CPUINFO;
use setup::{ArchKernelSetup, BootSetup, SetupReturn};
use crate::kernel::setup::ArmKernel;

pub struct ArmBoot;

impl ArmBoot {
    pub fn new() -> Self {
        return ArmBoot
    }

    pub fn setup(&self) {
        let early_io = self.early_io_init();
        let ld_mem = unsafe { self.linker_setup() };
        let cpu = self.early_cpu_init();

        if early_io.0.is_err() {
            panic!("{}", early_io.1);
        } else if ld_mem.0.is_err() {
            panic!("{}", ld_mem.1);
        } else if cpu.0.is_err() {
            panic!("{}", cpu.1);
        }

        if early_io.0.is_ok() {
            crate::early_printk!("{}\n", early_io.1);
        } else if ld_mem.0.is_ok() {
            crate::early_printk!("{}\n", ld_mem.1);
        } else if cpu.0.is_ok() {
            crate::early_printk!("{}\n", cpu.1);
        }
    }
}

impl BootSetup for ArmBoot {
    fn early_io_init(&self) -> SetupReturn {
        let arm_kernel = ArmKernel::new();
        return arm_kernel.serial_io_init();
    }

    unsafe fn linker_setup(&self) -> SetupReturn {
        extern "C" {
            static mut __bss_start: u64;
            static mut __bss_end: u64;
        }

        r0::zero_bss(&mut __bss_start, &mut __bss_end);

        if __bss_end != 0 {
            return (Err("__bss_end doesn't equal 0"), "Failed to setup linker memory");
        } else { return (Ok(()), "Cleared linker memory"); }
    }

    fn early_cpu_init(&self) -> SetupReturn {
        return (Ok(()), "");
    }
}

#[cfg(target_arch = "arm")]
pub mod boot32 {
    use crate::kernel::cpu::{info::CPUINFO, soc::soc_init};
    use setup::{BootSetup, SetupReturn};

    pub struct Arm32Boot;

    impl Arm32Boot {
        pub fn new() -> Self {
            return Arm32Boot;
        }

        pub fn setup(&self) {
            let dwt = self.disable_wdt();
            let cpuid = self.cpuid_init();

            if dwt.0.is_err() {
                panic!("{}", dwt.1);
            } else if cpuid.0.is_err() {
                panic!("{}", cpuid.1);
            }
        }
    }

    impl BootSetup for Arm32Boot {
        fn disable_wdt(&self) -> SetupReturn {
            return (Ok(()), "Successfully disabled Watch Dog Timer");
        }

        fn cpuid_init(&self) -> SetupReturn {
            return (Ok(()), "Successfully got and set CPU Id and info");
        }
    }


    pub fn arm32_boot_setup() {
        let arm32_boot = Arm32Boot::new();
        arm32_boot.setup();
    }
}
