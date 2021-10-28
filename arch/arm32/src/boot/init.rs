use crate::arm32_printk;
use super::cpu::cpu_init;
use cortex_m_rt_macros::entry;

#[entry]
fn init() -> ! {
    unsafe { cpu_init(); }
    arm32_printk!("CPU initialized");

    panic!("Kernel ended");
}
