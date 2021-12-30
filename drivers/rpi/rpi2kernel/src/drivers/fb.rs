use fb::{FbColor, FrameBufferGraphics};

pub struct Rpi2FrameBuffer {
    pub color: FbColor,
    pub x: usize,
    pub y: usize,
}

impl Rpi2FrameBuffer {
    pub fn new() -> Self {
        return Rpi2FrameBuffer {
            color: FbColor::new(0, 0, 0),
            x: 0,
            y: 0,
        };
    }

    pub fn init(&self) {

    }
}

impl FrameBufferGraphics for Rpi2FrameBuffer {
    fn pixel(&self, x: usize, y: usize, color: FbColor) {
        
    }
}

pub fn fb_init() {
    let mut fb = Rpi2FrameBuffer::new();

    fb.init();
}
