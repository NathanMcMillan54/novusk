use crate::kernel::cpu::{CPUINFO};

pub mod ints;
pub mod mm;
pub mod setup;
pub mod symbols;

pub struct CortexATarget;

impl CortexATarget {
    pub fn new() -> Self {
        return CortexATarget;
    }
}

pub fn cortex_a_init() {
    set_info();
}

fn set_info() {
    unsafe { CPUINFO.set("arm32", "Cortex A", Some(0)); }
}
