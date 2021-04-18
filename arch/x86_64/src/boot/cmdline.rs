use core::ptr::Unique;
use crate::drivers::text::{VGA_BUFFER_WIDTH, VGA_BUFFER_HEIGHT};
use crate::kernel::vga::{buffer::*, color::*};

pub unsafe fn cmdline_init() {
    let mut cmdline_writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Black, Color::Black),
        buffer: Unique::new_unchecked(0xb8000 as *mut _),
    };

    let mut i = 0;

    while i != VGA_BUFFER_HEIGHT * VGA_BUFFER_WIDTH {
        cmdline_writer.write_byte(b' ');
        i = i + 1;
    }
}
