use libcolor::ColorCode;
use novuskinc::fb::FrameBufferGraphics;

pub mod vga_80x25;

use vga_80x25::Vga80x25;

pub trait VgaMode {
    const WIDTH: usize = 0;
    const HEIGHT: usize = 0;
    const BUFFER_ADDRESS: usize = 0x0;

    fn write_byte(&mut self, byte: u8) {    }
}

#[derive(Copy, Clone)]
pub struct ScreenChar {
    character: u8,
    color: ColorCode,
}

pub struct VgaG {
    vga80x25: Vga80x25,
    pub mode: u32,
    pub size: (usize, usize),
}

impl VgaG {
    pub fn new() -> Self {
        return VgaG {
            vga80x25: Vga80x25::new(),
            mode: 0,
            size: (0, 0)
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
        self.size = (80, 25);

        self.clear_screen();
    }

    pub fn set_320x200x256_mode(&mut self) {

    }

    pub fn set_320x240x256_mode(&mut self) {

    }

    pub fn set_640x480x16_mode(&mut self) {

    }

    pub fn clear_screen(&mut self) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                self.graphics_write(b' ', x, y);
            }
        }
    }
}

impl FrameBufferGraphics for VgaG {
    fn graphics_write(&mut self, byte: u8, x: usize, y: usize) {
        self.vga80x25.write_byte(byte);
    }

    fn graphics_write_string(&mut self, string: &'static str, x: usize, y: usize) {
        for b in string.as_bytes() {
            self.graphics_write(*b, 0, 0)
        }
    }

    fn graphics_pixel(&self, color: u32, x: u32, y: u32) {

    }
}
