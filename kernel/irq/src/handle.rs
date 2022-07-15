use alloc::vec::Vec;
use novuskinc::irq::{device_irq_handler, IrqHandler};
use printk::printk;
use crate::chip::IRQCHIP;

#[no_mangle]
unsafe extern "C" fn empty_handler() -> i16 { 0 }

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
pub static mut IRQ_HANDLERS: Vec<IrqHandler> = vec![];

#[no_mangle]
pub unsafe extern "C" fn irq_handler() {
    printk!("\nIRQ\n");
    let irqn = IRQCHIP.get_irqn();
    // device_irq_handler(irqn);
}
