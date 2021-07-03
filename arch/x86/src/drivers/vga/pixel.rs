use libcolor::vga_colors::Color;
use super::VGA_ADDRESS;
use super::vga_80x25::*;

pub extern "C" fn _pixel(color: Color, x: usize) {
    let mut drawer = Vga80x25 {
        column_position: x,
        color_code: ColorCode::new(color, color),
        buffer: unsafe { &mut *(VGA_ADDRESS as *mut Buffer) }
    };

    drawer.write_byte(46);
}
