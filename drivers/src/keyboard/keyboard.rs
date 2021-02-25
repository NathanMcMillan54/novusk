use arch::ARCH;
use crate::keyboard::layout::default_layout;
use novusk_lib::*;
use pc_keyboard::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::keyboard::x86::{handlecontrol, scancode};

#[cfg(any(target_arch = "aarch64", target_arch = "aarch64"))]
use crate::keyboard::arm::{handlecontrol, scancode};

pub fn keyboard_init() {
    let mut keyboard= Keyboard::new(default_layout(),scancode(), handlecontrol());
    keyboard.add_byte(0x20);
    kinfo!("Keyboard initialized\n");
    kprint!("   Setup keyboard for {}\n", ARCH);
    match keyboard.add_byte(0x20) {
        Ok(Some(event)) => {
            kprint!("{:?}\n", event);
        }
        Ok(None) => {
            kprint!("Not enough data\n");
        }
        Err(e) => {
            kprint!("Error: {:?}\n", e);
        }
    }
}
