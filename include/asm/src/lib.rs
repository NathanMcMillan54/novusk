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

#[no_mangle]
pub unsafe extern "C" fn arch_asm_loop() {
    #[cfg(target_arch = "aarch64")]
    aarch64::wfe();

    #[cfg(target_arch = "arm")]
    arm32::wfe();

    #[cfg(target_arch = "x86_64")]
    x86_64::hlt();
}

#[no_mangle]
pub unsafe extern "C" fn disable_irqs() {
    #[cfg(target_arch = "arm")]
    arm32::ints::cpsid();

    #[cfg(target_arch = "x86_64")]
    x86_64::ints::cli();
}

#[no_mangle]
pub unsafe extern "C" fn enable_irqs() {
    #[cfg(target_arch = "x86_64")]
    x86_64::ints::sti();
}
