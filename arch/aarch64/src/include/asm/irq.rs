use asminc::irq::ArchIrqs;
use core::arch::asm;

#[no_mangle]
pub static mut ARCH_IRQS: ArchIrqs = ArchIrqs::new();

#[no_mangle]
pub unsafe extern "C" fn enable_irqs() {
    asm!("msr daifclr, #2");
}

#[no_mangle]
pub unsafe extern "C" fn diasble_irqs() {
    asm!("msr daifset, #2")
}
