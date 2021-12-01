use crate::kernel::cpu::{CPUINFO};

pub mod mm;
pub mod symbols;

pub fn cortex_a_init() {
    set_info();
}

fn set_info() {
    unsafe { CPUINFO.set("arm32", "Cortex A", Some(0)); }
}
