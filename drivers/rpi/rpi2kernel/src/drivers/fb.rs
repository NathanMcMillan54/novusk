use printk::console::KernelConsole;

pub struct Rpi2FrameBuffer {
    pub x: usize,
    pub y: usize,
}

impl Rpi2FrameBuffer {
    pub fn new() -> Self {
        return Rpi2FrameBuffer {
            x: 0,
            y: 0,
        };
    }

    pub fn init(&self) {

    }
}

pub fn fb_init() {
    let mut console = KernelConsole::new();
    let mut fb = Rpi2FrameBuffer::new();

    unsafe { console.init(); }
    fb.init();
}
