use crate::arm32_printk;
use init::kmain;
use super::board::BOARD;
use super::irq::irq_init;

pub unsafe fn arm_kernel_init() {
    irq_init();
    kinfo!("IRQs initialized\n");

    if BOARD.kernel_init == true {
        kmain::kernel_init();
        kinfo!("Novusk initialized\n");
    }
}
