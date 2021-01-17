use crate::x86::drivers::vga_text::SCREEN_SIZE;
use crate::x86::kernel::vga_buffer::{Buffer, Color, ColorCode, Writer};

pub fn fill_screen(color: Color) {
    let mut drawer = Writer {
        column_position: 0,
        color_code: ColorCode::new(color, color),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    let mut screen = 0;
    while screen != SCREEN_SIZE {
        screen = screen + 1;
        drawer.write_string(".");
    }
}
