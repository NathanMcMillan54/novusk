use cortex_m_rt::{exception, ExceptionFrame};
use time::cpu::CPU_TIME;
use crate::arm32_printk;

#[exception]
fn SysTick() {
    unsafe { CPU_TIME += 1; }
    arm32_printk!(".");
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    arm32_printk!("Hard Fault Interrupt\n");
    arm32_printk!("Exception frame:\n    {:?}", ef);

    panic!("Attempted to access none-existent address");
}

#[exception]
fn DefaultHandler(irq: i16) {
    arm32_printk!("Replacing interrupt: {}", irq);
}