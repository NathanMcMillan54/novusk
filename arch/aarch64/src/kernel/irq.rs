use asminc::aarch64::io::inb;
use novuskinc::irq::{device_irq_handler, irqchip_setup, irqchip_init, IrqChip};
use novuskinc::timer::device_timer_init;
use crate::early_printk;

extern "C" {
    fn irq_vector_init();
    pub static mut IRQCHIP: IrqChip;
}

pub unsafe fn aarch64_irq_setup() {
    irqchip_setup();
    irq_vector_init();
}

pub unsafe fn aarch64_irq_init() {
    device_timer_init();
    irqchip_init();
}
