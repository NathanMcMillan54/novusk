use core::ptr::write_volatile;
use novuskinc::keyboard::KeyboardInput;
use crate::Ps2Keyboard;

impl KeyboardInput for Ps2Keyboard {
    fn read_byte(&self) -> u8 {
        0
    }

    fn interrpret_byte(&self, byte: u8) {
        unsafe { write_volatile(0x520 as *mut u8, byte); }
    }
}

