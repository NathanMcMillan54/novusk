use cpu::arch::{cpu_arch, Arch};
use cpu::cpu::Cpu;
use crate::drivers::amd::amd_init;
use crate::drivers::intel::intel_init;

pub static mut ARCH: Arch = Arch::Unknown;
pub static mut BRAND: Cpu = Cpu::Unknown;

#[cfg(target_arch = "x86_64")]
unsafe fn x64_init() {
    use cpu::x64::cpu_brand;
    BRAND = cpu_brand();
    if BRAND == Cpu::AMD {
        amd_init();
    } else if BRAND == Cpu::Intel {
        intel_init();
    } else {
        return;
    }
}

#[cfg(target_arch = "x86")]
unsafe fn x86_init() {
    use cpu::x86::cpu_brand;
    BRAND = cpu_brand();
}

pub unsafe fn cpu_init() {
    ARCH = cpu_arch();

    #[cfg(target_arch = "x86_64")]
    x64_init();

    #[cfg(target_arch = "x86")]
    x86_init();
}