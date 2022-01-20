use crate::arm32_printk;
use crate::include::syscalls::syscalls_init;
// use init::kmain;
use super::board::BOARD;
use super::irq::irq_init;

pub unsafe fn arm_kernel_init() {
    irq_init();
    kinfo!("IRQs initialized\n");

    syscalls_init();
    kinfo!("ARM32 system calls initialized\n");

    if BOARD.kernel_init == true {
        //kmain::kernel_init();
        kinfo!("Novusk initialized\n");
    } else {
        arm32_printk!("\nStarting Board specific kernel...\n");
        BOARD.run_board_specific_kernel();
    }
}
