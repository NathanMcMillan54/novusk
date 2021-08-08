use kinfo::info::set_info;
use pc_keyboard::{ScancodeSet1, HandleControl};
use crate::keyboard_layout;

pub static mut KEYBOARD_PASSED: bool = false;

pub unsafe fn ps2_keyboard_test() -> bool {
    let mut kb = pc_keyboard::Keyboard::new(keyboard_layout(), ScancodeSet1, HandleControl::MapLettersToUnicode);
    match kb.add_byte(0x20) {
        Ok(Some(event)) => {
            printk!("{:?}", event);
            // ('_') <(Why are people the way they are?)
            return true;
        }
        Ok(None) => {
            set_info("not ok");
            kinfo!("Couldn't get input from test, expected byte 0x20 input from the \"D\" key");
            set_info("not ok");
            return false;
        }
        Err(err) => {
            set_info("not ok");
            kinfo!("Error decoding: {:?}", err);
            printk!("Try using a proper keyboard, or change keyboard layout at kernel compile time");
            set_info("not ok");
            return false;
        }
    }
}
