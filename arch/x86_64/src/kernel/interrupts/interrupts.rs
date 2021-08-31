use core::str::from_utf8;
use super::{PIC, index::InterruptIndex};
use notify::input::notify_keyboard_input;
use pc_keyboard::add_scancode;
use x86_64::instructions::port::{Port, PortWriteOnly, PortReadOnly};
use x86_64::structures::idt::InterruptStackFrame;
use crate::x86_printk;

pub extern "x86-interrupt" fn time_interrupt(stack_frame: InterruptStackFrame) {
    // TODO: In the future update a counter that keeps track of cpu ticks/time
    unsafe { PIC.lock().notify_end_of_interrupt(InterruptIndex::Timer as u8); }
}

// Hardware I/O
pub extern "x86-interrupt" fn keyboard_interrupt(stack_frame: InterruptStackFrame) {
    let mut keyboard_port = Port::new(0x60);
    let scancode = unsafe { keyboard_port.read() };

    //add_scancode(scancode);
    notify_keyboard_input(scancode);

    unsafe { PIC.lock().notify_end_of_interrupt(InterruptIndex::Keyboard as u8); }
}

pub extern "x86-interrupt" fn mouse_interrupt(stack_frame: InterruptStackFrame) {
    /* let mut mouse_port = PortReadOnly::new(0x60);
    let packet = unsafe { mouse_port.read() };

    MOUSE.lock().process_packet(packet);*/

    unsafe { PIC.lock().notify_end_of_interrupt(InterruptIndex::Mouse as u8); }
}
