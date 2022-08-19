use super::irq::irq_init;
use super::kernel::{RiscVKernel, RISCV_KERNEL};
use crate::include::syscalls::*;
use crate::mm::memory_init;
use setup::arch::ArchKernelSetup;
use setup::SetupReturn;

impl ArchKernelSetup for RiscVKernel {
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

        (Ok(()), "System functions setup successfully")
    }
}

#[no_mangle]
pub unsafe extern "C" fn setup_arch() {
    RISCV_KERNEL.setup();
}
