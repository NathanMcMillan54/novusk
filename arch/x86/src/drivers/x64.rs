use super::Drivers;
use crate::kernel::cpu::BRAND;
use crate::drivers::amd::amd_init;
use crate::drivers::intel::intel_init;
use crate::drivers::ps2::ps2_init;

struct X64Drivers;

impl Drivers for X64Drivers {
    unsafe fn cpu_driver_init(&self) {
        if BRAND == "AMD" {
            amd_init()
        } else if BRAND == "Intel" {
            intel_init();
        }
    }

    unsafe fn hardware_input_init(&self) {
        ps2_init();
    }
}

pub unsafe fn x64_drivers_init() {
    let driver_trait = X64Drivers;
    driver_trait.drivers_init();
}
