use crate::early_printk;

pub static mut TIMER_VALUE: f64 = 0.0;

unsafe fn update_timer(stack_frame: x86_64::structures::idt::InterruptStackFrame) {
    use super::i8259::PIC_8259;
    use super::irq::{PIC_OFFSETS, PIC_START};

    TIMER_VALUE += 0.06;

    PIC_8259.lock().notify_end_of_interrupt(PIC_START + PIC_OFFSETS[0]);
}

gen_x86_int!(time_irq, update_timer);
