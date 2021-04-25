use crate::text::vga::buffer::{Buffer, Writer};
use libn::color::vga::{Color, ColorCode};
use core::fmt::{Arguments, Write};

pub fn clear_screen() {
    let mut clear_writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Cyan, Color::Cyan),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    let mut i = 0;
    while i != 2000 {
        clear_writer.write_byte(b' ');
        i = i + 1;
    }
}

#[no_mangle]
pub extern "C" fn fkboot_msg(msg: Arguments) -> Arguments {
    let mut boot_writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Cyan),
        buffer: unsafe {  &mut *(0xb8000 as *mut Buffer) },
    };
    boot_writer.write_fmt(format_args!("{}{}", msg, "\n"));
    return msg;
}
