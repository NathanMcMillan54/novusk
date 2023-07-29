use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error: u64) -> ! {
    loop {  }
}

pub extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error: PageFaultErrorCode) {

}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {

}

pub extern "x86-interrupt" fn non_maskable_interrupt_handler(stack_frame: InterruptStackFrame) {

}
