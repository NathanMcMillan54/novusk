use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::exception;
use novuskinc::irq::*;

static mut SYSTICKS: u64 = 0;

#[exception]
fn SysTick() {
    unsafe { SYSTICKS += 1; }
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    let irq_ret = handle_irq(irqn);

    match irq_ret {
        IRQH_SUCCESS => return,
        IRQH_FAILED => panic!("IRQ {} failed\n", irqn),
        IRQH_NOT_EXISTENT => printk!("IRQ {} is unimplemented\n", irqn),
        _ => panic!("IRQ returned {}", irq_ret)
    };
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    printk!("Exception frame: {:?}", ef);
    panic!("Hard fault");
}

#[exception]
fn SVCall() {
    printk!("svc");
}
