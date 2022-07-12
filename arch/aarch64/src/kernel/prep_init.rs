use crate::early_printk;
use novuskinc::irq::irqchip_init;
use cortex_a::registers::CurrentEL;
use tock_registers::interfaces::Readable;

#[no_mangle]
pub unsafe extern "C" fn arch_prepare_init() {
    irqchip_init();

    early_printk!("Exception Level: {:?}", CurrentEL.get())
}
