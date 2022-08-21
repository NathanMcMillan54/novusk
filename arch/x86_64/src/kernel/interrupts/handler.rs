use kinfo::status::{KStatus, set_status};
use crate::include::asm::hlt;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
use crate::kinfo::InfoDisplay;

pub extern "x86-interrupt" fn break_point_handler(isf: InterruptStackFrame) {
    kinfo!(KStatus {
        status: "not ok",
        should_panic: false,
        panic_message: None,
        main_message: "Interrupt breakpoint",
        messages: None,
    });

    early_printk!("Interrupt stack frame:\n{:?}\n", isf);
}

pub extern "x86-interrupt" fn page_fault_handler(isf: InterruptStackFrame, error_code: PageFaultErrorCode) {
    kinfo!(KStatus {
        status: "not ok",
        should_panic: false,
        panic_message: None,
        main_message: "Page fault",
        messages: None,
    });

    early_printk!("Interrupt stack frame:\n{:?}\n", isf);
    early_printk!("Error code: {:?}", error_code);
}

pub extern "x86-interrupt" fn double_fault_handler(isf: InterruptStackFrame, error_code: u64) -> ! {
    kinfo!(KStatus {
        status: "not ok",
        should_panic: false,
        panic_message: None,
        main_message: "Double fault",
        messages: None,
    });
    early_printk!("Stack frame: {:?}\n", isf);
    early_printk!("Error code: {}\n", error_code);

    panic!("Double fault error");
}
