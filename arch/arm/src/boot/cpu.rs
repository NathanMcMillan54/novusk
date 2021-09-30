use crate::{cortex_m3, cortex_m4};
use cortex_m::Peripherals;

pub struct Cpuid {
    base: u32,
}

impl Cpuid {
    pub fn new() -> Self {
        let cpuid = Peripherals::take().unwrap().CPUID;
        let cpu_base = cpuid.base.read();

        return Cpuid { base: cpu_base };
    }

    pub fn get_base(&mut self) -> u32 {
        return self.base;
    }

    pub fn as_str(&mut self) -> &'static str {
        return match self.base {
            1091551793 => "Cortex M3",
            1091551808 => "Cortex M4",
            _ => "Unknown",
        }
    }
}

pub(crate) fn cpu_init() {
    let mut cpuid = Cpuid::new();

    if cpuid.as_str() == "Cortex M3" {
        cortex_m3::cortex_m3_init();
    } else if cpuid.as_str() == "Cortex M4" {
        cortex_m4::cortex_m4_init();
    }
}
