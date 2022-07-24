use asminc::irq::ArchIrqs;
use core::arch::asm;

#[no_mangle]
pub static mut ARCH_IRQS: ArchIrqs = ArchIrqs::new();

#[no_mangle]
pub unsafe extern "C" fn disable_irqs() {
    // asm!("cpsid");
}

#[no_mangle]
pub unsafe extern "C" fn enable_irqs() {
    // asm!("cpsie");
}
