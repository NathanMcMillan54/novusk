use kinfo::status::set_status;
use crate::x86_printk;
use crate::include::asm::hlt;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

pub extern "x86-interrupt" fn break_point_handler(isf: InterruptStackFrame) {
    unsafe {
        set_status("not ok");
        kinfo!("Interrupt breakpoint:\n");
    }

    x86_printk!("    {:?}\n", isf);
}

pub extern "x86-interrupt" fn page_fault_handler(isf: InterruptStackFrame, error_code: PageFaultErrorCode) {
    unsafe {
        set_status("not ok");
        kinfo!("Page fault:\n");
    }

    x86_printk!("    Stack frame: {:?}\n", isf);
    x86_printk!("    Error code: {:?}\n", error_code);

    panic!("Page fault error");
}

pub extern "x86-interrupt" fn double_fault_handler(isf: InterruptStackFrame, error_code: u64) -> ! {
    unsafe {
        set_status("not ok");
        kinfo!("Double fault:\n");
    }

    x86_printk!("    Stack frame: {:?}\n", isf);
    x86_printk!("    Error code: {}\n", error_code);

    panic!("Double fault error");
}
