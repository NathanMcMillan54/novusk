use crate::early_printk;
use novuskinc::irq::irqchip_init;

#[no_mangle]
pub unsafe extern "C" fn arch_prepare_init() {
    irqchip_init();
}
