use super::Drivers;
use crate::drivers::intel::intel_init;

struct X86Drivers;

impl Drivers for X86Drivers {
    unsafe fn cpu_driver_init(&self) {
        // Almost all x86 CPUs are Intel and x86 CPUID hasn't been worked on much so always assume it's Intel
        intel_init();
    }
}

pub unsafe fn x86_drivers_init() {
    let driver_trait = X86Drivers;
    driver_trait.drivers_init();
}
