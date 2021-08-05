use super::cpu::id::{get_cpuid, BRAND};
use super::kernel::*;
use super::modules::x86_modules_init;
use crate::drivers::drivers::drivers_init;

pub unsafe fn x86_kernel_init() {
    get_cpuid();
    kinfo!("Got cpuid");
    x86_printk!("    CPU brand: {}", BRAND);

    drivers_init();

    x86_modules_init();
}
