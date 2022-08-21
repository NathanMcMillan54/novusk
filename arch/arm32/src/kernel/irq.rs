use crate::target::ints::enable_interrupts;
use super::irqs::device_specific_irq_init;

pub unsafe fn irq_init() {
    enable_interrupts();
    device_specific_irq_init();
}
