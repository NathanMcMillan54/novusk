use crate::early_printk;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

// Panic handler
#[path = "panic.rs"]
pub mod panic;

pub extern "x86-interrupt" fn break_point_handler(isf: InterruptStackFrame) {
    early_printk!("\n\nKernel breakpoint\n");
    early_printk!("Interrupt stack frame: {:?}\n", isf);

    panic!("Break point");
}

#[no_mangle]
pub extern "x86-interrupt" fn page_fault_handler(isf: InterruptStackFrame, pg_error: PageFaultErrorCode) {
    early_printk!("\n\nKernel page fault error\n");
    early_printk!("Interrupt stack frame: {:?}\n", isf);
    early_printk!("Page fault error code: {:?}\n", pg_error);

    loop {  }
}

#[no_mangle]
pub(crate) extern "x86-interrupt" fn double_fault(isf: InterruptStackFrame, int_error: u64) -> ! {
    early_printk!("\n\nKernel double fault\n");
    early_printk!("Interrupt stack frame: {:?}\n", isf);
    early_printk!("Interrupt error code: {}\n", int_error);

    loop {  }
}

