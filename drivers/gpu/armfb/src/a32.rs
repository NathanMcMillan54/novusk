use fb::{FbColor, FrameBufferGraphics};

pub struct A32Fb;

impl A32Fb {
    pub fn new() -> Self {
        return A32Fb;
    }
}

impl FrameBufferGraphics for A32Fb {
    fn pixel(&self, x: usize, y: usize, color: FbColor) {

    }

    fn clear_screen(&self, color: FbColor) {

    }

    fn write_char(&self, x: usize, y: usize, color: FbColor, c: char) {

    }

    fn write_string(&self, x: usize, y: usize, color: FbColor, string: &str) {
        for b in string.as_bytes() {
            self.write_char(x, y, color, *b as char);
        }
    }
}
