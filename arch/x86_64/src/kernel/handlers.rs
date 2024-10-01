use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
//use crate::early_printk;

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error: u64) -> ! {
    /*early_printk!("----- Double fault -----\n");
    early_printk!("Interrupt stack frame: {:?}\n", stack_frame);
    early_printk!("Error code: {}\n", error);*/
    loop {  }
}

pub extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error: PageFaultErrorCode) {
    /*early_printk!("----- Page fault -----\n");
    early_printk!("Interrupt stack frame: {:?}\n", stack_frame);
    early_printk!("Error code: {:?}\n", error);*/
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    /*early_printk!("----- Break point -----\n");
    early_printk!("Interrupt stack frame: {:?}\n", stack_frame);*/
}

pub extern "x86-interrupt" fn non_maskable_interrupt_handler(stack_frame: InterruptStackFrame) {
    /*early_printk!("----- Non maskable interrupt -----\n");
    early_printk!("Interrupt stack frame: {:?}\n", stack_frame);*/
}
