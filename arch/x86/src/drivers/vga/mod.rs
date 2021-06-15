pub mod init;
pub mod macros;
pub mod writer;

use core::fmt::{Arguments, Write};
use libcolor::vga_colors::Color;
use writer::{Buffer, ColorCode, VGAWriter};

pub unsafe fn _vga_write(fmt: Arguments) {
    let mut writer = VGAWriter {
        column_position: 0,
        color_code: ColorCode::new(Color::LightGray, Color::Black),
        buffer: &mut *(0xb8000 as *mut Buffer)
    };

    writer.write_fmt(format_args!("{}{}", fmt, "\n"));
}

#[no_mangle]
pub unsafe extern "C" fn _vga_pixel(color: Color) {
    let mut writer = VGAWriter {
        column_position: 0,
        color_code: ColorCode::new(color, color),
        buffer: &mut *(0xb8000 as *mut Buffer)
    };

    writer.write_byte(*b" ".as_ptr());
}
