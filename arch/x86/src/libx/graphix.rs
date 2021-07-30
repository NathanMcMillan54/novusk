use libcolor::vga_colors::Color;
use crate::drivers::vga::pixel::_pixel;
use crate::x86_printk;

pub struct Graphix;

impl Graphix {
    pub fn new() -> Self {
        return Self;
    }

    pub fn pixel(&mut self, color: Color, x: usize) {
        unsafe { _pixel(color, x); }
    }

    pub fn line(&mut self, color: Color, sx: usize, ex: usize) {
        // Draw pixels between the starting X and ending X
        for i in sx..ex {
            self.pixel(color, i);
        }
    }

    pub fn clear_screen(&mut self) {
        for i in 0..81 {
            self.line(Color::Black, 0, 26);
            unsafe { x86_printk!("") }
        }
    }
}
