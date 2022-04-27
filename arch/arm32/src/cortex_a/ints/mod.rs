use core::arch::asm;

pub mod common;

pub fn enable_interrupts() {
    unsafe { asm!("cpsie if"); }
}
