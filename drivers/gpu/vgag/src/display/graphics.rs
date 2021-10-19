use vga::writers::{Graphics640x480x16, GraphicsWriter};
use crate::color::convert_usize_to_color16;

pub(crate) fn graphics_640x480_write(x: usize, y: usize, color: usize, string: &str) {
    let mode = Graphics640x480x16::new();

    for (space, byte) in string.chars().enumerate() {
        mode.draw_character(x + space * 8, y, byte, convert_usize_to_color16(color));
    }
}

pub(crate) fn graphics_640x480_pixel(x: usize, y: usize, color: usize) {
    let color = convert_usize_to_color16(color);

    let mode = Graphics640x480x16::new();
    mode.set_pixel(x, y, color);
}
