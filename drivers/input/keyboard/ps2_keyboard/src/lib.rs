#![no_std]
#![feature(abi_x86_interrupt)]

#[macro_use] extern crate asminc;
#[macro_use] extern crate novuskinc;

pub mod input;
pub mod layouts;

use layouts::Layout;
use novuskinc::keyboard::*;

#[cfg(not(target_arch = "x86_64"))]
compile_error!("This driver is meant to run on x86_64");

pub static mut PS2_KEYBOARD: Keyboard = Keyboard::new("PS2");

pub struct Ps2Keyboard {
    pub ps2: bool,
    pub layout: Layout,
}

impl Ps2Keyboard {
    pub fn new() -> Self {
        // This will be changable eventually
        return Ps2Keyboard {
            ps2: false,
            layout: Layout::Us104(pc_keyboard::layouts::Us104Key),
        }
    }
}

pub unsafe fn ps2_keyboard_init() {
    PS2_KEYBOARD.set_kb_driver(&Ps2Keyboard {
        ps2: false,
        layout: Layout::Us104(pc_keyboard::layouts::Us104Key),
    } as &dyn KeyboardInput);
}
