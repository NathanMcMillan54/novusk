use super::handlers::*;
use x86_64::structures::idt::InterruptDescriptorTable;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub unsafe fn set_idt() {
    IDT.double_fault.set_handler_fn(double_fault);
    IDT.page_fault.set_handler_fn(page_fault_handler);
    IDT.breakpoint.set_handler_fn(break_point_handler);
}

pub unsafe fn idt_init() {
    IDT.load();
}
