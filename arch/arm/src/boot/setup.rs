use crate::kernel::cpu::info::CPUINFO;
use setup::{ArchKernelSetup, BootSetup, SetupReturn};
use crate::early_printk;
use crate::kernel::cpu::soc::soc_init;
use crate::kernel::setup::ArmKernel;

pub struct ArmBoot;

impl ArmBoot {
    pub fn new() -> Self {
        return ArmBoot
    }

    pub fn setup(&self) {
        #[cfg(not(feature = "cortex_m"))]
        let ld_mem = unsafe { self.linker_setup() };

        let early_io = self.early_io_init();
        let cpu = self.early_cpu_init();
        let cpuid = self.cpuid_init();

        #[cfg(not(feature = "cortex_m"))]
        if ld_mem.0.is_err() {
            panic!("{}", ld_mem.1);
        } else { crate::early_printk!("{}", ld_mem.1); }

        if early_io.0.is_err() {
            panic!("{}", early_io.1);
        } else if cpu.0.is_err() {
            panic!("{}", cpu.1);
        } else if cpuid.0.is_err() {
            panic!("{}", cpuid.1);
        }

        if early_io.0.is_ok() {
            crate::early_printk!("{}\n", early_io.1);
        } else if cpuid.0.is_ok() {
            crate::early_printk!("{}\n", cpu.1);
        } else if cpu.0.is_ok() {
            crate::early_printk!("{}\n", cpuid.1);
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

    fn cpuid_init(&self) -> SetupReturn {
        unsafe { CPUINFO.architecture = "ARM" }

        #[cfg(target_arch = "aarch64")]
        unsafe { CPUINFO.bits = 64; }

        return (Ok(()), "Some CPU info set");
    }
}

#[cfg(target_arch = "arm")]
pub mod boot32 {
    use crate::kernel::cpu::{info::CPUINFO};
    use setup::{BootSetup, SetupReturn};

    pub struct Arm32Boot;

    impl Arm32Boot {
        pub fn new() -> Self {
            return Arm32Boot;
        }

        pub fn setup(&self) {
            let cpu = self.early_cpu_init();

            if cpu.0.is_err() {
                panic!("{}", cpu.1);
            } else { crate::early_printk!("{}\n", cpu.1); }
        }
    }

    impl BootSetup for Arm32Boot {
        fn disable_wdt(&self) -> SetupReturn {

            return (Ok(()), "Successfully disabled Watch Dog Timer");
        }

        fn cpuid_init(&self) -> SetupReturn {
            unsafe {
                CPUINFO.bits = 32;
                // Assuming it's on a Cortex M cpu
                CPUINFO.base_address = Some(0xE000_ED00 as usize as u32);
                CPUINFO.brand_name = "Unknown";
            }

            return (Ok(()), "Successfully got and set CPU Id and info");
        }

        fn early_cpu_init(&self) -> SetupReturn {
            use crate::kernel::cpu::soc::soc_init;

            if self.disable_wdt().0.is_err() {
                return (Err("WDT error"), "Failed to disable DWT");
            } else if self.cpuid_init().0.is_err() {
                return (Err("CPU Id error"), "Failed to set CPU info and id");
            }

            /*let soc_ret = unsafe { soc_init() };

            if soc_ret != 0 {
                return (Err("SOC init error"), "Failed to initialize SOC");
            }*/

            return (Ok(()), "Early CPU initialization successful");
        }
    }


    pub fn arm32_boot_setup() {
        let arm32_boot = Arm32Boot::new();
        arm32_boot.setup();
    }
}
