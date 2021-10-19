#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate lazy_static;

pub mod color;
pub mod display;
pub mod switch;

pub use color::{convert_usize_to_color16, Color16};
pub use display::VgaDisplay;

use fb::Fb;
use spin::Mutex;
use vga::writers::{Text80x25, Graphics640x480x16};
use crate::VgaModes::Graphics640x480;

lazy_static! {
    pub static ref VGAG: Mutex<VgaG> = Mutex::new(VgaG::new(VgaModes::default()));
}

pub struct VgaG {
    pub fb: Fb,
    pub mode: VgaModes,
    pub display: VgaDisplay,
}

impl VgaG {
    pub fn new(vga_mode: VgaModes) -> Self {
        return VgaG {
            fb: Fb::new("VGA", vga_mode.address().unwrap()),
            mode: vga_mode,
            display: VgaDisplay::new(vga_mode),
        };
    }

    pub fn init(&mut self) {
        self.mode.switch();
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum VgaModes {
    Text80x25,
    Graphics320x200,
    Graphics320x240,
    Graphics640x480,
    None,
}

impl VgaModes {
    pub fn address(self) -> Option<usize> {
        if self == VgaModes::Text80x25 {
            return Some(0xb8000);
        } else if self == VgaModes::Graphics320x200 || self == VgaModes::Graphics320x240 || self == VgaModes::Graphics640x480 {
            return Some(0xa0000)
        } else { return None }
    }
}

impl Default for VgaModes {
    fn default() -> Self {
        return VgaModes::None;
    }
}

pub fn convert_usize_to_vgamode(vgamode: usize) -> VgaModes {
    return match vgamode {
        0 => VgaModes::Text80x25,
        1 => VgaModes::Graphics320x240,
        2 => VgaModes::Graphics320x240,
        3 => VgaModes::Graphics640x480,

        _ => VgaModes::None,
    };
}
