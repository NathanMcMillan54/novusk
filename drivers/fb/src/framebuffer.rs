use super::fb_color::FbColor;

pub struct Fb {
    pub name: &'static str,
    pub address: usize,
}

impl Fb {
    pub fn new(fb_name: &'static str, fb_address: usize) -> Self {
        return Fb {
            name: fb_name,
            address: fb_address,
        };
    }

    pub unsafe fn fb_write(&mut self, write: u8) {
        self.address = write as usize;
    }
}

pub trait FrameBufferGraphics {
    fn pixel(&self, x: usize, y: usize, color: FbColor) {

    }

    fn write_char(&self, x: usize, y: usize, color: FbColor, c: char) {

    }

    fn write_string(&self, x: usize, y: usize, color: FbColor, string: &str) {
        let mut ux = x;
        let mut uy = y;

        for b in string.as_bytes() {
            self.write_char(ux, uy, color, *b as char);
            ux += 4;
            uy += 4;
        }
    }
}
