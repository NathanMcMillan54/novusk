use vga::writers::{Text80x25, TextWriter, ScreenCharacter};
use crate::color::convert_usize_to_color16;
use vga::colors::{TextModeColor, Color16};

pub(crate) fn text_80x25_write(x: usize, y: usize, color: usize, string: &str) {
    let mode = Text80x25::new();

    let mut nx = x;
    let mut ny = y;

    for b in string.as_bytes() {
        let screen_char = ScreenCharacter::new(*b, TextModeColor::new(convert_usize_to_color16(color), Color16::Black));

        if b == &b'\n' { ny += 1; nx = x; } else {
            mode.write_character(nx, ny, screen_char);
            nx += 1;
        }
    }
}

pub(crate) fn text_80x25_pixel(x: usize, y: usize, color: usize) {
    let mode = Text80x25::new();
    let color = convert_usize_to_color16(color);
    let pixel = ScreenCharacter::new(b' ', TextModeColor::new(color, color));

    mode.write_character(x, y, pixel);
}
