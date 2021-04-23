use core::fmt::{Arguments, Write};
use drivers::x86_64::vga::{buffer::*, WRITER};
use super::time::current_time;
use libn::color::vga::*;

#[no_mangle]
pub unsafe extern "C" fn write(bytes: &[u8], size: u8) -> u8 {
    WRITER.lock().write_byte(*bytes.as_ptr());
    return size;
}

#[no_mangle]
pub unsafe extern "C" fn write_fmt(args: Arguments) -> Arguments {
    WRITER.lock().write_fmt(args).unwrap();
    return args;
}

#[no_mangle]
pub unsafe extern "C" fn kernel_time() -> f64 {
    current_time()
}

#[no_mangle]
pub unsafe extern "C" fn draw(color: Color) -> Color {
    let mut kdrawer = Writer {
        column_position: 0,
        color_code: ColorCode::new(color, color),
        buffer: &mut *(0xb8000 as *mut Buffer)
    };
    kdrawer.write_byte(b' ');
    return color;
}
