use super::vga::{buffer::*, color::*};
use core::ptr::Unique;

pub unsafe fn write(write_byte: u8, byte_size: u8) -> u8 {
    let mut kwriter = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: Unique::new_unchecked(0xb8000 as *mut _),
    };
    kwriter.write_byte(write_byte);
    return byte_size;
}

pub unsafe fn draw(pos: u8, color: Color) {
    let mut drawer = Writer {
        column_position: pos as usize,
        color_code: ColorCode::new(color, color),
        buffer: Unique::new_unchecked(0xb8000 as *mut _),
    };
    drawer.write_byte(b' ');
}

#[no_mangle]
pub unsafe extern "Rust" fn syscall(call: i32, arg1: u8, arg2: u8) {
    if call == 0 {
        write(arg1, arg2);
    } else if call == 1 {
        // Default Cyan
        draw(arg1, Color::Cyan);
    } // else { panick!("{} - Unknown system call - read Documentation/kernel/syscalls.txt for list of system call numbers\n", call); }
}
