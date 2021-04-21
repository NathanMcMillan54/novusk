pub mod buffer;
pub mod color;

use buffer::*;
use color::*;
use kernel::die;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

pub(crate) fn vga_error() {
    static TEXT: &[u8] = b"Error: VGA screen buffer isn't 25x80";
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in TEXT.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xf;
        }
    }
}

pub unsafe fn init() {
    if BUFFER_HEIGHT + BUFFER_WIDTH != 105 {
        vga_error();
        die();
    } else {
        return;
    }
}
