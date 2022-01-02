use super::irq::irq_init;
use novuskinc::kernel::syscalls::table::DEFAULT_NAME;
use crate::rv_printk;
use crate::include::syscalls::*;
use crate::mm::memory_init;
use setup::arch::ArchKernelSetup;
use setup::SetupReturn;

struct RiscvKernel;

impl RiscvKernel {
    pub fn new() -> Self {
        return RiscvKernel;
    }

    pub fn setup(&self) {
        let irq_ret = self.irq_init();
        let mem_ret = self.memory_setup();

        let mut dev_ret: SetupReturn;
        //let mut sys_ret: SetupReturn;

        unsafe {
            dev_ret = self.device_init();
            //sys_ret = self.sys_setup();
        }

        rv_printk!("Finished RISCV kernel setup\n");
        rv_printk!("Running on: {}\n", dev_ret.1);
        rv_printk!("{}\n{}\n", irq_ret.1, mem_ret.1);
    }
}

impl ArchKernelSetup for RiscvKernel {
    fn memory_setup(&self) -> SetupReturn {
        if memory_init().is_ok() {
            return (Ok(()), "Successfully setup memory");
        } else { return (Err("An error ouccourd while initializing memory"), "Failed to setup memory"); }
    }

    fn irq_init(&self) -> SetupReturn {
        unsafe { irq_init(); }

        return (Ok(()), "Enabled IRQs");
    }

    unsafe fn sys_setup(&self) -> SetupReturn {
        syscalls_init();

        if SYSCALL_TABLE.systable_name != DEFAULT_NAME {
            return (Ok(()), "System functions setup successfully");
        } else { return (Err("An error occurred while setting up system functions"), "System functions setup unsuccessfully"); }
    }
}

unsafe fn start_main() {
    extern "C" {
        fn kernel_main();
    }

    rv_printk!("Starting kernel main...\n");
    kernel_main();
}

pub fn setup_riscv_kernel() {
    let kernel_setup = RiscvKernel::new();

    kernel_setup.setup();

    //unsafe { start_main(); }
}
