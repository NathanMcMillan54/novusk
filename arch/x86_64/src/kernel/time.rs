use novuskinc::irq::notify_irq;
use time::kernel::{update_time, KERNEL_TIME};
use crate::early_printk;

// The timer IRQ gets called about once every half second
pub(crate) const TIME_RATE: f64 = 0.05;

unsafe fn update_timer(stack_frame: x86_64::structures::idt::InterruptStackFrame) {
    use super::irq::irqns::IRQ_1;

    update_time();

    notify_irq(IRQ_1);
}

gen_x86_int!(time_irq, update_timer);
