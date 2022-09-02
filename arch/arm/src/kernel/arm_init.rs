use crate::include::syscalls::syscalls_init;
use init::kmain;
use kinfo::{InfoDisplay, status::KStatus};
use super::board::BOARD;
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

    core::arch::asm!("svc 0x1");
    core::arch::asm!("svc 0x1");
    core::arch::asm!("svc 0x3");

    if BOARD.kernel_init == true {
        kmain::kernel_init();
        kinfo!(KStatus {
            status: "ok",
            should_panic: false,
            panic_message: None,
            main_message: "Novusk initialized",
            messages: None,
        });
    } else {
        early_printk!("\nStarting Board specific kernel...\n");
        BOARD.run_board_specific_kernel();
    }
}
