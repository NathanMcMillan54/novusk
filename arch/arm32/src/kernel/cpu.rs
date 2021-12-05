use crate::arm32_printk;
use cpu::CpuInfo;

pub static mut CPUINFO: CpuInfo = CpuInfo::emtpy();

pub trait CpuSetup {
    fn irq_setup(&self) {

    }
}

pub fn cpu_setup() {
    #[cfg(feature = "cortex_m")]
    crate::target::CortexMTarget::new().irq_setup();

    #[cfg(feature = "cortex_a")]
    crate::target::CortexATarget::new().irq_setup();
}
