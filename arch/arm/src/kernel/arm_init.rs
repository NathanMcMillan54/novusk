use super::irq::irqs_init;

pub unsafe fn arm_kernel_init() {
    irqs_init();
}
