use core::fmt::{Arguments, Write};
use core::ptr::Unique;
use crate::akernel::vga::{buffer::*, color::*};

#[no_mangle]
pub extern "C" fn red(args: Arguments) {
    let mut red_writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Red, Color::Black),
        buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
    };
    red_writer.write_fmt(args);
}
