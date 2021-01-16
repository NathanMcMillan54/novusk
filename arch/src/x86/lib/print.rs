use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::x86::kernel::vga_buffer::*;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

pub fn x86_print(arg: fmt::Arguments) {
    write!(WRITER.lock(), "{}", arg);
}
