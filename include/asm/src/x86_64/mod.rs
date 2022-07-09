use core::arch::asm;

/// Functions and macros for x86_64 IRQs
pub mod ints;

/// Runs the ``hlt`` instruction
pub unsafe fn hlt() {
    asm!("hlt");
}
