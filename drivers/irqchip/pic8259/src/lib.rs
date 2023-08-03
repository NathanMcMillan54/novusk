#![no_std]

use pic8259::ChainedPics;
use spin::Mutex;

static mut PIC8259: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(32, 40) });

#[no_mangle]
unsafe extern "C" fn irqchip_init() {
    PIC8259.lock().initialize();
}

#[no_mangle]
unsafe extern "C" fn notify_irq(irq: u8) {
    PIC8259.lock().notify_end_of_interrupt(irq);
}
