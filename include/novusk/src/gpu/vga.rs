use core::fmt::{Arguments, Write};
use vgag::Color16;
use vgag::display::VgaDisplay;

pub fn vga_write(x: usize, y: usize, fmt: Arguments) {
    let mut vga = VgaDisplay::new(x, y, Color16::White);
    vga.write_fmt(fmt);
}
