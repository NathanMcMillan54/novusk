//global_asm!(include_str!("x86_id.S"));

use super::BRAND;

pub unsafe fn get_cpuid() {

    // It's likely GenuineIntel
    BRAND = "Intel";
}

