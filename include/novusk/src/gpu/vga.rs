use core::fmt::{Arguments, Write};
use core::str::from_utf8_unchecked;
use crate::define_syscall;
use vgag::{VGAG, VgaDisplay, VgaModes};

pub fn vga_write(x: usize, y: usize, string: &str) {
    let mut vga = unsafe { VgaDisplay::new(VGAG.mode) };

    (vga.write_fun)(x, y, 15, string);
}

unsafe fn vga_sys_write(byte: u8, tl: u8, sys_arg3: u8) -> u8 {
    if VGAG.mode == VgaModes::Text80x25 {
        VGAG.x_pos += 1;
    } else { VGAG.x_pos += 1 * 8; }

    if byte == b'\n' {
        if VGAG.mode == VgaModes::Text80x25 {
            VGAG.x_pos -= tl as usize;
            VGAG.y_pos += 1;
        } else {
            for i in 0..VGAG.chars_written {
                VGAG.x_pos -= 1 * 8;
            }

            VGAG.y_pos += 9;
        }

        VGAG.chars_written = 1;
    }

    VGAG.chars_written += 1;

    vga_write(VGAG.x_pos, VGAG.y_pos, from_utf8_unchecked([byte].as_slice()));

    return 0;
}

unsafe fn vga_sys_write_init(set_x: u8, set_y: u8, sys_arg3: u8) -> u8 {
    VGAG.x_pos = set_x as usize;
    VGAG.y_pos = set_y as usize;

    return sys_arg3;
}

define_syscall!(sys_write, vga_sys_write);
define_syscall!(sys_write_init, vga_sys_write_init);
