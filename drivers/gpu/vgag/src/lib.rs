#![no_std]

pub use vga::colors::Color16;
pub use vga::writers::{Graphics320x200x256, Graphics320x240x256, Graphics640x480x16, Text40x25, Text40x50, Text80x25};

pub mod display;
pub mod resolution;
use resolution::set_resolution;

pub fn vgag_init() {
    set_resolution(Graphics640x480x16::new());
}
