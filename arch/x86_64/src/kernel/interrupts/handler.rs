use kinfo::info::set_info;
use crate::x86_printk;
use crate::include::asm::hlt;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

pub extern "x86-interrupt" fn break_point_handler(isf: InterruptStackFrame) {
    unsafe {
        set_info("not ok");
        kinfo!("Interrupt breakpoint:");
    }

    x86_printk!("    {:?}", isf);
}

pub extern "x86-interrupt" fn page_fault_handler(isf: InterruptStackFrame, error_code: PageFaultErrorCode) {
    unsafe {
        set_info("not ok");
        kinfo!("Page fault:");
    }

    x86_printk!("    Stack frame: {:?}", isf);
    x86_printk!("    Error code: {:?}", error_code);

    panic!("Page fault error");
}

pub extern "x86-interrupt" fn double_fault_handler(isf: InterruptStackFrame, error_code: u64) -> ! {
    unsafe {
        set_info("not ok");
        kinfo!("Double fault:");
    }

    x86_printk!("    Stack frame: {:?}", isf);
    x86_printk!("    Error code: {}", error_code);

    panic!("Double fault error");
}
