#![no_std]
#![feature(panic_info_message)]

use novuskinc::irq::{device_irq_handler, IrqChip};
use printk::printk;

#[path = "../../panic.rs"]
pub mod panic;

extern "C" {
    static mut IRQCHIP: IrqChip;
}

#[no_mangle]
pub unsafe extern "C" fn irq_handler() {
    printk!("\nIRQ\n");
    let irqn = (IRQCHIP.irqn)();

    device_irq_handler(irqn);
}
