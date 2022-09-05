use crate::target::ints::enable_interrupts;
use novuskinc::irq::irqchip_init;

pub unsafe fn irq_init() {
    enable_interrupts();
    irqchip_init();
}
