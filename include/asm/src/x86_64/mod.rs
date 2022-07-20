use core::arch::asm;

/// Functions and macros for x86_64 IRQs
pub mod ints;

/// Port I/O
pub mod io;

/// Runs the ``hlt`` instruction
pub unsafe fn hlt() {
    asm!("hlt");
}
