pub mod common;

pub fn enable_interrupts() {
    unsafe { asm!("cpsie if"); }
}
