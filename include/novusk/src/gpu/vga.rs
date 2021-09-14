use core::fmt::{Arguments, Write};
use crate::define_syscall;
use vgag::Color16;
use vgag::display::VgaDisplay;

pub fn vga_write(x: usize, y: usize, fmt: Arguments) {
    let mut vga = VgaDisplay::new(x, y, Color16::White);
    vga.write_fmt(fmt);
}

// -----------------
// Write/sys_write for x86_64
//
// System call for writing
#[cfg(target_arch = "x86_64")]
pub fn write(sys_arg: u8) -> u8 {
    let mut vga = VgaDisplay::new(0, 0, Color16::White);
    vga.write_char(sys_arg as char);
    return 0;
}

#[cfg(target_arch = "x86_64")]
define_syscall!(sys_write, write);