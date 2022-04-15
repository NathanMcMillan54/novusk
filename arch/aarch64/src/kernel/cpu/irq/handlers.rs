use novuskinc::irq::device_irq_handler;

#[no_mangle]
pub extern "C" fn DefaultHandler(irq: i16) {
    crate::early_printk!("IRQ {} was called", irq);
}

#[no_mangle]
pub unsafe extern "C" fn handle_irqs() {
    let irqn = 0;
    DefaultHandler(irqn);
    device_irq_handler(irqn);
}
