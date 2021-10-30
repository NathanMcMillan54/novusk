use crate::arm32_printk;
use crate::kernel::io::ARM32IO;
use cortex_m::Peripherals;
use cortex_m::peripheral::CPUID;

pub unsafe fn cortex_m_init() {
    let mut cpu_peripherals = Peripherals::take().unwrap();

    let cpuid = cpu_peripherals.CPUID;
}
