use super::{GpuGraphics, GPUGRAPHICS};

impl GpuGraphics {
    pub fn pixel(&mut self, x: usize, y: usize, color: usize) {
        let pixel = self.pixel_fun;

        pixel(x, y, color);
    }
}

pub fn _pixel(x: usize, y: usize, color: usize) {
    let mut gg = GPUGRAPHICS.lock();
    gg.pixel(x, y, color);
}

#[macro_export]
macro_rules! pixel {
    ($x:ident, $y:ident, $color:ident) => {$crate::pixel::_pixel($x, $y, $color)};
}
