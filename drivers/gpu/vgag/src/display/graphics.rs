use vga::writers::{Graphics640x480x16, GraphicsWriter};
use crate::VGAG;
use crate::color::convert_usize_to_color16;

pub(crate) fn graphics_640x480_write(x: usize, y: usize, color: usize, string: &str) {
    let mode = Graphics640x480x16::new();

    let mut nx = x;
    let mut ny = y;

    for b in string.as_bytes() {
        mode.draw_character(nx, ny, *b as char, convert_usize_to_color16(color));
        nx += 1 * 8;

        if b == &b'\n' {
            nx = x;
            ny += 9;
        }
    }
}

pub(crate) fn graphics_640x480_pixel(x: usize, y: usize, color: usize) {
    let color = convert_usize_to_color16(color);

    let mode = Graphics640x480x16::new();
    mode.set_pixel(x, y, color);
}
