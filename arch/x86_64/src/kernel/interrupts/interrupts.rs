use super::{PIC, index::InterruptIndex};
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn keyboard_interrupt(stack_frame: InterruptStackFrame) {
    unsafe { PIC.lock().notify_end_of_interrupt(InterruptIndex::Keyboard as u8); }
}

pub extern "x86-interrupt" fn mouse_interrupt(stack_frame: InterruptStackFrame) {
    unsafe { PIC.lock().notify_end_of_interrupt(InterruptIndex::Mouse as u8); }
}
