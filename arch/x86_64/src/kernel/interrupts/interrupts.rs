use core::str::from_utf8;
use super::{PIC, index::InterruptIndex};
use time::{cpu, kernel};
use x86_64::instructions::port::{Port, PortWriteOnly, PortReadOnly};
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn time_interrupt(stack_frame: InterruptStackFrame) {
    // cpu::update_cpu_time(amd64_timer::ticks_amd());
    kernel::update_kernel_time(1);
    unsafe { PIC.lock().notify_end_of_interrupt(InterruptIndex::Timer as u8); }
}

pub extern "x86-interrupt" fn keyboard_interrupt(stack_frame: InterruptStackFrame) {
    let mut keyboard_port = Port::new(0x60);
    let scancode: u8 = unsafe { keyboard_port.read() };

    //notify_keyboard_input(scancode);

    unsafe { PIC.lock().notify_end_of_interrupt(InterruptIndex::Keyboard as u8); }
}
