#![no_std]

#[macro_use] extern crate novuskinc;

use pic8259::ChainedPics;
use spin::Mutex;

// This is only meant to hold PIC1 and PIC2 which is for a timer and ps2 keyboard input
// In the future a library won't be used and it will be properly implemented
pub static mut PIC_8259: Mutex<ChainedPics> = unsafe { Mutex::new(ChainedPics::new(32, 40)) };

#[no_mangle]
pub unsafe extern "C" fn irqchip_init() {
    PIC_8259.lock().initialize();
}

#[no_mangle]
pub unsafe extern "C" fn notify_irq(irqn: u8) {
    PIC_8259.lock().notify_end_of_interrupt(irqn);
}
