use cortex_m_rt::{exception, ExceptionFrame};
use time::cpu::CPU_TIME;

#[exception]
fn SysTick() {
    unsafe { CPU_TIME += 1; }
    printk!(".");
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    printk!("Hard Fault Interrupt\n");
    printk!("Exception frame:\n    {:?}", ef);

    panic!("Attempted to access a none-existent address");
}

#[exception]
unsafe fn DefaultHandler(irq: i16) {
    printk!("Replacing interrupt: {}", irq);
}
