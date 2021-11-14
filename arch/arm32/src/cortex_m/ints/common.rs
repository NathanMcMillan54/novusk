use crate::arm32_printk;
use cortex_m_rt::exception;
use time::cpu::CPU_TIME;

#[exception]
unsafe fn SysTick() {
    CPU_TIME += 1;
}
