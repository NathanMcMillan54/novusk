use core::fmt::Write;
use crate::early_printk;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

// Panic handler
#[path = "../../../../kernel/panic.rs"]
pub mod panic;

#[no_mangle]
pub extern "C" fn device_indicate_panic() {
    use super::video_vga::*;
    use libcolor::{Color16, ColorCode};

    let mut vga = EarlyVga {
        column_position: 2,
        color_code: ColorCode::new(Color16::White as u8, Color16::Red as u8),
        buffer: unsafe { &mut *(BUFFER_ADDRESS as *mut Buffer) }
    };

    vga.write_str("!!!");
}

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

