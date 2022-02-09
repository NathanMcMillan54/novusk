#[derive(Debug, Copy, Clone)]
pub struct FrameBuffer {
    pub name: &'static str,
    pub fb_size: (u8, u8),
    pub fb_addr: *mut u8,
}

impl FrameBuffer {
    pub fn new() -> Self {
        return FrameBuffer {
            name: "None",
            fb_size: (0, 0),
            fb_addr: 0x0 as *mut u8,
        }
    }

    pub const fn empty() -> Self {
        return FrameBuffer {
            name: "None",
            fb_size: (0, 0),
            fb_addr: 0x0 as *mut u8,
        };
    }

    pub fn set(&mut self, fb_name: &'static str, size: (u8, u8), addr: *mut u8) {
        self.name = fb_name;
        self.fb_size = size;
        self.fb_addr = addr;
    }
}
