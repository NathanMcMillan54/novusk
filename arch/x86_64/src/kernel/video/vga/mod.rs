pub mod color;
pub mod init;
pub mod writer;
pub mod vga_80x25;
pub mod vga_320x200;

#[cfg(not(feature = "vga_0xa"))]
pub use vga_80x25::*;

#[cfg(feature = "vga_0xa")]
pub use vga_320x200::*;

use core::fmt::{Arguments, Write};
use crate::kernel::vga::color::ColorCode;
use libcolor::vga_colors::Color;

pub static mut VGA_ADDRESS_STR: &str = "0xb8000";

pub fn _vga_print(fmt: Arguments) {
    let mut vga_writer = writer::VgaWriter::new(VGA_ADDRESS, BUFFER_WIDTH, BUFFER_HEIGHT, ColorCode::new(Color::LightGray, Color::Black));

    vga_writer.write_fmt(fmt);
}
