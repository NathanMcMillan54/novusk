use core::arch::asm;
use crate::kernel::kernel::{check_cpuid, set_CPU_BRAND};

// Read ``Documentation/x86_64/cpus.md`` for Novusk's requirements for running on an x86_64 CPU.
pub unsafe fn validate_cpu() -> bool {
    if check_cpuid() == 1 {
        panic!("");
    }
//    set_CPU_BRAND();

    return true;
}
