use crate::early_printk;
use super::irq::aarch64_irq_init;

#[no_mangle]
pub unsafe extern "C" fn arch_prepare_init() {
    aarch64_irq_init();
}
