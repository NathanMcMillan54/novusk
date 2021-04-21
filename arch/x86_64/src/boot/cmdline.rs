use super::{SCREEN_HEIGHT, SCREEN_WIDTH};
use drivers::text::vga::{buffer::*, color::*};
use drivers::x86_64::vga::WRITER;


pub unsafe fn cmdline_init() {
    let mut i = 0;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Black, Color::Black),
        buffer: &mut *(0xb8000 as *mut Buffer),
    };

    while i != SCREEN_HEIGHT * SCREEN_WIDTH {
        writer.write_byte(b' ');
        i = i + 1;
    }
    return;
}
