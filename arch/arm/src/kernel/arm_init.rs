use crate::include::syscalls::syscalls_init;
use kinfo::{InfoDisplay, status::KStatus};
use crate::kernel::cpu::ArmCpuRegisters;
use super::irq::irq_init;

pub unsafe fn arm_kernel_init() {
    irq_init();
    kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "IRQs initialized",
        messages: None,
    });

    /*syscalls_init();
    kinfo!("ARM32 system calls initialized\n");*/


}
