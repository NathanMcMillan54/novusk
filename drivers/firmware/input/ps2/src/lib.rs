#![no_std]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;

pub mod keyboard;
pub mod test;

#[cfg(target_arch = "x86_64")]
pub mod mouse;

#[cfg(feature = "us_layout")]
pub fn keyboard_layout() -> pc_keyboard::layouts::Us104Key {
    return pc_keyboard::layouts::Us104Key;
}

#[cfg(feature = "uk_layout")]
pub fn keyboard_layout() -> pc_keyboard::layouts::Uk105Key {
    return pc_keyboard::layouts::Uk105Key;
}

#[cfg(feature = "custom_layout")]
pub fn keyboard_layout() -> pc_keyboard::layouts::Dvorak104Key {
    return pc_keyboard::layouts::Dvorak104Key;
}
