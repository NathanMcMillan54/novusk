use kinfo::info::set_info;
use pc_keyboard::{HandleControl, Keyboard, ScancodeSet1, layouts};
use crate::x86_printk;

unsafe fn keyboard_test() {
    let mut kb = pc_keyboard::Keyboard::new(layouts::Uk105Key, ScancodeSet1, HandleControl::MapLettersToUnicode);
    match kb.add_byte(0x20) {
        Ok(Some(event)) => {
            // ('_') <( I'm still alive. (in case you were wondering)
        }
        Ok(None) => {
            set_info("not ok");
            kinfo!("Couldn't get input from test, expected byte 0x20 input from the \"D\" key");
        }
        Err(err) => {
            set_info("not ok");
            kinfo!("Error decoding: {:?}", err);
        }
    }
}

pub unsafe fn ps2_test() {
    keyboard_test();
}
