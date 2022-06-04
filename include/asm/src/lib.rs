#![no_std]

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "arm")]
pub mod arm32;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

/// Runs ``nop`` instruction which is supported on most architectures
pub unsafe fn nop() {
    core::arch::asm!("nop");
}
