use libcolor::vga_colors::Color;
use crate::kernel::vga::color::ColorCode;
use crate::kernel::vga::writer::VgaWriter;
use super::{BUFFER_HEIGHT, BUFFER_WIDTH, VGA_ADDRESS};

pub fn vga_init() {
    let mut writer = VgaWriter::new();

    writer.write_byte(b'a');
}
