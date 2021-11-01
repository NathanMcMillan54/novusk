use crate::arm32_printk;
use crate::kernel::io::ARM32IO;
use cortex_m::Peripherals;
use cortex_m::peripheral::CPUID;
pub use cortex_m_rt::heap_start;

pub mod model;

pub unsafe fn cortex_m_init() -> (&'static str, u32) {
    let mut cpu_peripherals = Peripherals::take().unwrap();

    let cpuid = cpu_peripherals.CPUID;
    let cpu_model = model::get_model(cpuid.base.read());

    return (cpu_model, cpuid.base.read());
}
