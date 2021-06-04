use pc_keyboard::{HandleControl, ScancodeSet1, layouts};

pub unsafe fn test_keyboard() {
    let mut kb = pc_keyboard::Keyboard::new(layouts::Uk105Key, ScancodeSet1, HandleControl::Ignore);
    match kb.add_byte(0x20) {
        Ok(Some(event)) => {
            printk!("Event {:?}", event);
        }
        Ok(None) => {
            printk!("Need more data");
        }
        Err(e) => {
            printk!("Error decoding: {:?}", e);
        }
    }
}
