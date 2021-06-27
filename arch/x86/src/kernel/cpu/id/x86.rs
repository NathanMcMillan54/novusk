global_asm!(include_str!("x86_id.S"));

use super::BRAND;

extern "C" {
    fn get_x86_cpuid();
}

pub unsafe fn get_cpuid() {
    get_x86_cpuid();

    // It's likely GenuineIntel
    BRAND = "Intel";
}

