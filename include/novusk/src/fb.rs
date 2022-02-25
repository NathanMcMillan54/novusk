pub trait FrameBufferGraphics {
    fn graphics_write(&mut self, byte: u8, x: usize, y: usize) {

    }

    fn graphics_write_string(&mut self, string: &'static str, x: usize, y: usize) {
        let mut nx = x;
        let mut ny = y;

        for b in string.as_bytes() {
            nx += 1;

            if *b == b'\n' { ny += 1; }

            self.graphics_write(*b, nx, ny);
        }
    }

    fn graphics_pixel(&self, color: u32, x: u32, y: u32) {

    }
}

struct EmptyGraphics;

impl FrameBufferGraphics for EmptyGraphics {
    fn graphics_write(&mut self, byte: u8, x: usize, y: usize) {

    }
}

#[derive(Copy, Clone)]
pub struct FrameBuffer<'a> {
    pub name: &'static str,
    pub fb_size: (u8, u8),
    pub fb_addr: *mut u8,
    pub graphics: &'a (dyn FrameBufferGraphics + 'a),
}

impl<'a> FrameBuffer<'a> {
    pub fn new() -> Self {
        return FrameBuffer {
            name: "None",
            fb_size: (0, 0),
            fb_addr: 0x0 as *mut u8,
            graphics: &EmptyGraphics as &dyn FrameBufferGraphics,
        };
    }

    pub const fn empty() -> Self {
        return FrameBuffer {
            name: "None",
            fb_size: (0, 0),
            fb_addr: 0x0 as *mut u8,
            graphics: &EmptyGraphics as &dyn FrameBufferGraphics,
        };
    }

    pub fn set(&mut self, fb_name: &'static str, size: (u8, u8), addr: *mut u8, fb_graphics: &'a dyn FrameBufferGraphics) {
        self.name = fb_name;
        self.fb_size = size;
        self.fb_addr = addr;
        self.graphics = fb_graphics;
    }
}
