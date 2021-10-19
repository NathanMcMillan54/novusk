use alloc::boxed::Box;
use core::any::Any;
use crate::{VgaModes, convert_usize_to_vgamode};
use vga::colors::Color16;
use vga::vga::Vga;
use vga::writers::{GraphicsWriter, TextWriter, Text80x25, Graphics320x200x256, Graphics320x240x256, Graphics640x480x16};

pub fn switch_graphics_mode<T: GraphicsWriter<Color16> + Any>(mode: T) {
    mode.set_mode();
    mode.clear_screen(Color16::Black);
}

fn switch_text_mode<T: TextWriter + Any>(mode: T) {
    mode.set_mode();
    mode.clear_screen();
}

impl VgaModes {
    pub fn switch(self) {
        if self == VgaModes::Text80x25 {
            switch_text_mode(Text80x25::new());
        } else if self == VgaModes::Graphics320x200 {
            // switch_graphics_mode(Graphics320x200x256::new().c);
        } else if self == VgaModes::Graphics320x240 {
            // switch_graphics_mode(Graphics320x240x256::new());
        } else if self == VgaModes::Graphics640x480 {
            switch_graphics_mode(Graphics640x480x16::new());
        } else {  }
    }
}

pub extern "C" fn vga_switch(mode: usize) {
    let vgamode = convert_usize_to_vgamode(mode);

    vgamode.switch();
}
