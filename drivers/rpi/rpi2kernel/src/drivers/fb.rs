use armfb::a64::A64Fb;
use fb::{FbColor, FrameBufferGraphics};

pub fn fb_init() {
    let mut fb = A64Fb::new();

    fb.init();
    fb.clear_screen(FbColor::new(0x4d0000, 0, 0));
}
