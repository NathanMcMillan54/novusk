use core::arch::asm;
use novuskinc::irq::notify_irq;
use printk::printk;
use crate::early_printk;

unsafe fn ps2_input(stack_frame: x86_64::structures::idt::InterruptStackFrame) {
    use super::irq::irqns::IRQ_2;

    notify_irq(IRQ_2);
}

gen_x86_int!(ps2_keyboard, ps2_input);
