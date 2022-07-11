use core::arch::asm;
use novuskinc::irq::irqchip_setup;

pub mod handlers;

#[no_mangle]
pub unsafe extern "C" fn aarch64_irq_setup() {
    irqchip_setup();
}
