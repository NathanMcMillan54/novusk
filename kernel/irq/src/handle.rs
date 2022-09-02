use novuskinc::irq::{empty_handler, IRQH_NOT_EXISTENT, IrqHandler};
use crate::chip::IRQCHIP;

#[cfg(feature = "12irqs")]
pub const MAX_IRQS: usize = 12;

#[cfg(feature = "16irqs")]
pub const MAX_IRQS: usize = 16;

#[cfg(feature = "24irqs")]
pub const MAX_IRQS: usize = 24;

#[cfg(feature = "32irqs")]
pub const MAX_IRQS: usize = 32;

#[cfg(feature = "48irqs")]
pub const MAX_IRQS: usize = 48;

#[cfg(feature = "64irqs")]
pub const MAX_IRQS: usize = 64;

#[cfg(feature = "120irqs")]
pub const MAX_IRQS: usize = 120;

#[cfg(feature = "240irqs")]
pub const MAX_IRQS: usize = 240;

#[no_mangle]
pub static mut IRQ_HANDLERS: [IrqHandler; MAX_IRQS] = [
    IrqHandler {
        irqn: (MAX_IRQS + 1) as i16,
        irqh: empty_handler,
    }; MAX_IRQS
];

#[no_mangle]
pub unsafe extern "C" fn add_irq(irqh: IrqHandler) {
    IRQ_HANDLERS[irqh.irqn as usize] = irqh;
}

#[no_mangle]
pub unsafe extern "C" fn irq_handler() {
    let irqn = IRQCHIP.get_irqn();
    // device_irq_handler(irqn);
}

#[no_mangle]
pub unsafe extern "C" fn handle_irq(irqn: i16) -> i16 {
    let irq_ret = (IRQ_HANDLERS[irqn as usize].irqh)();

    return irq_ret;
}
