use libcolor::vga_colors::Color;
use crate::kernel::kernel::{CPU_BRAND, set_CPU_BRAND};
use crate::kernel::vga::color::ColorCode;
use crate::kernel::vga::writer::VgaWriter;
use super::{BUFFER_HEIGHT, BUFFER_WIDTH, VGA_ADDRESS};

pub fn vga_init() {
    let mut writer = VgaWriter::new();

    unsafe { writer.write_byte(b'a'); }
}
