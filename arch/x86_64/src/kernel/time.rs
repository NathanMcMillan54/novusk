use novuskinc::irq::notify_irq;
use crate::early_printk;

pub static mut TIMER_VALUE: f64 = 0.0;

unsafe fn update_timer(stack_frame: x86_64::structures::idt::InterruptStackFrame) {
    use super::irq::irqns::IRQ_1;

    TIMER_VALUE += 0.06;

    printk::printk!(".");

    notify_irq(IRQ_1);
}

gen_x86_int!(time_irq, update_timer);
