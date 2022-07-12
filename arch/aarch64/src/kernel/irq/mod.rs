use core::arch::asm;
use novuskinc::irq::irqchip_setup;

extern "C" {
    fn irq_vector_init();
}

pub mod handlers;

#[no_mangle]
pub unsafe extern "C" fn aarch64_irq_setup() {
    irqchip_setup();
}
