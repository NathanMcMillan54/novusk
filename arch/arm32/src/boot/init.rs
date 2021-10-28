use crate::arm32_printk;
use crate::kernel::setup::setup_atm32_kernel;
use super::cpu::cpu_init;
use cortex_m_rt_macros::entry;

#[entry]
fn init() -> ! {
    unsafe { cpu_init(); }
    kinfo!("CPU initialized");

    arm32_printk!("\nSetting up kernel...\n");
    unsafe { setup_atm32_kernel(); }

    panic!("Kernel ended");
}
