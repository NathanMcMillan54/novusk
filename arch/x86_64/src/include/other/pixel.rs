use libcolor::vga_colors::Color;
use crate::kernel::vga::{BUFFER_HEIGHT, BUFFER_WIDTH, VGA_ADDRESS};
use crate::kernel::vga::color::ColorCode;
use crate::kernel::vga::writer::{Buffer, ScreenChar};

struct PixelDrawer {
    pub x_pos: usize,
    pub y_pos: usize,
    pub color: ColorCode,
    pub vga_buffer: &'static mut Buffer,
}

impl PixelDrawer {
    pub fn draw_pixel(&mut self) {
        let row = self.y_pos;
        let col = self.x_pos;
        let color_code = self.color;

        self.vga_buffer.chars[row][col].write(ScreenChar {
            ascii_character: 46,
            color_code,
        });
    }
}


#[link_name = "apixel"]
#[no_mangle]
pub extern "C" fn _vga_pixel(color: Color, x: usize, y: usize) {
    let mut drawer = PixelDrawer {
        x_pos: x,
        y_pos: y,
        color: ColorCode::new(color, color),
        vga_buffer: unsafe { &mut *(VGA_ADDRESS as *mut Buffer) }
    };

    drawer.draw_pixel();
}
