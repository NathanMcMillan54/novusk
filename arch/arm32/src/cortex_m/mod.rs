use crate::kernel::cpu::{CPUINFO};
use cortex_m::Peripherals;

pub mod mm;

pub fn cortex_m_init() {
    set_info();
}

fn set_info() {
    let peripherals = Peripherals::take().unwrap();
    let cpuid = peripherals.CPUID;

    unsafe { CPUINFO .set("arm32", "Cortex M", Some(cpuid.base.read())); }
}
