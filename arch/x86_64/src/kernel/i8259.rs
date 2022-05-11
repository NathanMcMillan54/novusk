use super::irq::{PIC_OFFSETS, PIC_START};
use pic8259::ChainedPics;
use spin::Mutex;

// This is only meant to hold PIC1 and PIC2 which is for a timer and ps2 keyboard input
// In the future a library won't be used and it will be properly implemented
pub static mut PIC_8259: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_START + PIC_OFFSETS[0], PIC_START + PIC_OFFSETS[1]) });

#[no_mangle]
pub unsafe extern "C" fn pic_8259_init() {
    PIC_8259.lock().initialize();
}
