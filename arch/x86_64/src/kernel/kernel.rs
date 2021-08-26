pub use super::x86_init::x86_kernel_init;
pub use crate::x86_printk;

use super::vga::{writer::VgaWriter, BUFFER_HEIGHT, BUFFER_WIDTH, VGA_ADDRESS};
use libcolor::vga_colors::Color;
use crate::kernel::vga::color::ColorCode;

#[no_mangle]
pub extern "C" fn writes(s: &str) {
    let mut writer = VgaWriter::new(VGA_ADDRESS, BUFFER_WIDTH, BUFFER_HEIGHT, ColorCode::new(Color::LightGray, Color::Black));

    writer.write_string(s);
}
