use cortex_m_rt::exception;
use time::cpu::CPU_TIME;
use crate::arm32_printk;

#[exception]
fn SysTick() {
    unsafe { CPU_TIME += 1; }
}
