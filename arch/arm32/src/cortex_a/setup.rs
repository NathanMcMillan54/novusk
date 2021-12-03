use cortex_m::Peripherals;
use crate::kernel::cpu::CpuSetup;
use super::CortexATarget;

impl CpuSetup for CortexATarget {
    fn irq_setup(&self) {

    }
}
