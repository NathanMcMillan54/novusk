use kinfo::info::set_info;
use pc_keyboard::{HandleControl, Keyboard, ScancodeSet1, layouts};
use super::keyboard::setup::keyboard_layout;
use crate::x86_printk;

pub static mut KEYBOARD_PASSED: bool = false;

unsafe fn keyboard_test() {
    let mut kb = pc_keyboard::Keyboard::new(keyboard_layout(), ScancodeSet1, HandleControl::MapLettersToUnicode);
    match kb.add_byte(0x20) {
        Ok(Some(event)) => {
            // ('_') <( I'm still alive. (in case you were wondering )
            KEYBOARD_PASSED = true;
        }
        Ok(None) => {
            set_info("not ok");
            kinfo!("Couldn't get input from test, expected byte 0x20 input from the \"D\" key");
            KEYBOARD_PASSED = false;
        }
        Err(err) => {
            set_info("not ok");
            kinfo!("Error decoding: {:?}", err);
            x86_printk!("Try using a proper keyboard, or change keyboard layout at kernel compile time");
            KEYBOARD_PASSED = false;
        }
    }
}

pub unsafe fn ps2_test() {
    keyboard_test();
}
