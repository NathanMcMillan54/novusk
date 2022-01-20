use crate::arm32_printk;
use super::irqs::device_specific_irq_init;

pub unsafe fn irq_init() {
    device_specific_irq_init();
}
