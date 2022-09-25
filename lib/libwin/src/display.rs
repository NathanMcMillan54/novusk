
/*impl Window {
    pub fn display(&mut self) {
        if self.open == false {
            return;
        }

        let (sx, sy) = self.size.unwrap();
        let (px, py) = self.pos;

        for wy in 0..sy {
            for wx in 0..sx {
                unsafe { graphics_pixel(px as usize + wx as usize, py as usize + wy as usize, self.color); }
            }
        }

        for y in 0..16 {
            for x in 0..sx {
                unsafe { graphics_pixel(px as usize + x as usize, py as usize + y as usize, LIGHT_GRAY); }
            }
        }

        unsafe { graphics_write(px as usize + 5, py as usize + 4, WHITE, self.title.unwrap()); }
    }
}
*/