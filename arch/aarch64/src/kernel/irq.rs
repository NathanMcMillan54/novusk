use novuskinc::irq::irqchip_setup;
use crate::early_printk;

extern "C" {
    fn irq_vector_init();
}

pub unsafe fn aarch64_irq_setup() {
    irqchip_setup();
    irq_vector_init();
}
