use super::irq::irqs_init;

#[no_mangle]
pub unsafe extern "C" fn arch_prepare_init() {
    irqs_init();
}
