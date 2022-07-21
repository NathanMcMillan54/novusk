use kinfo::status::KStatus;
use crate::early_printk;
use super::irq::aarch64_irq_init;

#[no_mangle]
pub unsafe extern "C" fn arch_prepare_init() {
    aarch64_irq_init();
    kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        message1: "IRQs initialized",
        message2: None
    });
}
