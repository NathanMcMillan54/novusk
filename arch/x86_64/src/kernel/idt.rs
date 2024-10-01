use x86_64::instructions::port::Port;
use x86_64::structures::idt::{Entry, HandlerFunc, InterruptDescriptorTable, InterruptStackFrame};
use super::handlers::*;

#[repr(u8)]
enum Interrupts {
    Timer = 32,
    Keyboard,
}

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

extern "x86-interrupt" fn timer_handler(stack_frame: InterruptStackFrame) {
    //unsafe { notify_irq(32); }
}

extern "x86-interrupt" fn keyboard_handler(stack_frame: InterruptStackFrame) {
    let b: u8 = unsafe { Port::new(0x60).read() };

    //unsafe { notify_irq(33); }
}

extern "x86-interrupt" fn mouse_handler(stack_frame: InterruptStackFrame) {
    let b: u8 = unsafe { Port::new(0x60).read() };

    //unsafe { notify_irq(44); }
}

pub unsafe fn idt_init() {
    IDT.breakpoint.set_handler_fn(breakpoint_handler);
    IDT.page_fault.set_handler_fn(page_fault_handler);
    IDT.double_fault.set_handler_fn(double_fault_handler);
    IDT.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt_handler);
    IDT[32].set_handler_fn(timer_handler);
    IDT[33].set_handler_fn(keyboard_handler);
    IDT[44].set_handler_fn(mouse_handler);

    IDT.load();
}
