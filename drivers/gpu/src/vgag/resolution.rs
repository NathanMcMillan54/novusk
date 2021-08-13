use core::any::Any;
use core::fmt::Debug;
use vga::colors::Color16;
use vga::writers::{GraphicsWriter, TextWriter};
use super::{Graphics320x200x256, Graphics320x240x256, Graphics640x480x16, Text40x25, Text40x50, Text80x25};

pub fn set_resolution<T: GraphicsWriter<Color16> + Any>(res: T) {
    res.set_mode();
    res.clear_screen(Color16::Black);
}

pub fn set_text_mode<T: TextWriter + Any>(mode: T) {
    mode.set_mode();
    mode.clear_screen();
}
