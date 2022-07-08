use novuskinc::irq::{device_specific_irqs_init, irqchip_init};

pub unsafe fn irqs_init() {
    device_specific_irqs_init();
    irqchip_init();
}
