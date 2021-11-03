use crate::{Window, graphics::graphics::{graphics_write, graphics_pixel}};

impl Window {
    pub fn display(&mut self) {
        let (x, y) = self.size.unwrap();

        for wy in 0..y {
            for wx in 0..x {
                graphics_pixel(wx as usize, wy as usize, self.color);
            }
        }
    }
}
