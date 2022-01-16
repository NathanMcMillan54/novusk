use crate::kernel::cpu::{CPUINFO};
use cortex_m::Peripherals;

pub mod ints;
pub mod mm;
pub mod setup;

pub struct CortexMTarget;

impl CortexMTarget {
    pub fn new() -> Self {
        return CortexMTarget;
    }
}

pub fn cortex_m_init() {
    set_info();
}

fn set_info() {
    let peripherals = Peripherals::take().unwrap();
    let cpuid = peripherals.CPUID;

    unsafe { CPUINFO.set("arm32", "Cortex M", Some(cpuid.base.read())); }
}
