use vga::writers::{Text80x25, TextWriter, ScreenCharacter};
use crate::color::convert_usize_to_color16;
use vga::colors::{TextModeColor, Color16};

pub(crate) fn text_80x25_write(x: usize, y: usize, color: usize, string: &str) {
    let mode = Text80x25::new();
    let screen_char = ScreenCharacter::new(unsafe { *string.as_ptr() }, TextModeColor::new(convert_usize_to_color16(color), Color16::Black));

    mode.write_character(x, y, screen_char)
}

pub(crate) fn text_80x25_pixel(x: usize, y: usize, color: usize) {
    let mode = Text80x25::new();
    let color = convert_usize_to_color16(color);
    let pixel = ScreenCharacter::new(b' ', TextModeColor::new(color, color));

    mode.write_character(x, y, pixel);
}
