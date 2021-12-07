#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate novuskinc;

pub mod color;
pub mod display;
pub mod switch;
pub mod types;

pub use color::*;
pub use display::{graphics_pixel, graphics_write, VgaDisplay};

use fb::Fb;
use spin::Mutex;
use types::*;
use vga::writers::{Text80x25, Graphics640x480x16};
use crate::VgaModes::Graphics640x480;

pub static mut VGAG: VgaG = VgaG {
    switch: 0,
    mode: VgaModes::Text80x25,
    first_init: true,
};

pub struct VgaG {
    pub switch: u32,
    pub first_init: bool,
    pub mode: VgaModes,
}

impl VgaG {
    fn check_switch(&mut self) -> VgaModes {
        return match self.switch {
            0 => VgaModes::Text80x25,
            1 => VgaModes::Graphics320x200,
            2 => VgaModes::Graphics320x240,
            3 => VgaModes::Graphics640x480,
            4 => {
                self.switch = 0;
                VgaModes::Text80x25
            },

            _ => VgaModes::None,
        };
    }

    pub fn init(&mut self) {
        if self.first_init == true {
            self.switch = 3;
        }

        let mode = self.check_switch();
        self.mode = mode;

        self.mode.switch();
    }
}

fn vgag_init() {
    unsafe {
        let mode = VGAG.check_switch();

        if mode == VgaModes::None {
            panic!("{} is an invalid switch value", VGAG.switch);
        }

        VGAG.init();
        VGAG.switch += 1;
        VGAG.first_init = false;
    }
}

module_init!(gpug_init, vgag_init);

fn vgag_end() {
    unsafe {
        if VGAG.check_switch() == VgaModes::None {
            VGAG.switch = 0;
        }
    }
}

module_end!(gpug_end, vgag_end);
