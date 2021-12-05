use super::irq::irq_init;

pub unsafe fn arm_kernel_init() {
    irq_init();
    kinfo!("IRQs initialized");
}
