use x86_64::structures::idt::{InterruptDescriptorTable};
use super::handlers::*;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        unsafe { idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(0); }
        idt.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt_handler);

        idt
    };
}

pub unsafe fn idt_init() {
    IDT.load();
}
