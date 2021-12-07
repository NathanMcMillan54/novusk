use init::kmain;
use super::irq::irq_init;

pub unsafe fn arm_kernel_init() {
    irq_init();
    kinfo!("IRQs initialized\n");

    kmain::kernel_init();
    kinfo!("Novusk initialized\n");
}
