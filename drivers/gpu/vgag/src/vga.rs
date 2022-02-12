use novuskinc::fb::FrameBufferGraphics;
use vga::vga::VGA;

pub struct VgaG {
    pub mode: u32,
    pub fb_address: *mut u32,
    pub size: (u32, u32),
}

impl VgaG {
    pub fn new() -> Self {
        return VgaG {
            mode: 0,
            fb_address: 0xb8000 as *mut u32,
            size: (80, 25),
        };
    }

    pub fn set_mode(&mut self, mode: u32) {
        match mode {
            0 => self.set_80x25_mode(),
            1 => self.set_320x200x256_mode(),
            2 => self.set_320x240x256_mode(),
            3 => self.set_640x480x16_mode(),

            _ => self.set_80x25_mode(),
        }
    }

    pub fn set_80x25_mode(&mut self) {
        self.mode = 0;
        self.fb_address = 0xb8000 as *mut u32;
        self.size = (80, 25);

        self.clear_screen();
    }

    pub fn set_320x200x256_mode(&mut self) {

    }

    pub fn set_320x240x256_mode(&mut self) {

    }

    pub fn set_640x480x16_mode(&mut self) {

    }

    pub fn clear_screen(&self) {
        let mut fb = self.fb_address;

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                unsafe { fb.offset(y as isize + x as isize).write_volatile(y); }
            }
        }
    }
}

impl FrameBufferGraphics for VgaG {
    fn graphics_write(&mut self, byte: u8, x: usize, y: usize) {
        
    }

    fn graphics_write_string(&mut self, string: &'static str, x: usize, y: usize) {
        
    }

    fn graphics_pixel(&self, color: u32, x: u32, y: u32) {

    }
}
