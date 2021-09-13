use core::fmt::{Arguments, Write, Result};
use core::iter::Enumerate;
use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};

pub struct VgaDisplay {
    pub x: usize,
    pub y: usize,
    pub color: Color16,
    pub mode: Graphics640x480x16,
}

impl VgaDisplay {
    pub fn new(vga_x: usize, vga_y: usize, vga_color: Color16) -> Self {
        return VgaDisplay {
            x: vga_x,
            y: vga_y,
            color: vga_color,
            mode: Graphics640x480x16::new()
        };
    }

    // This is going to replace arch/x86_64/src/kernel/etc... Vga80x25::write_string
    pub fn write_string(&mut self, string: &str) {
        for (space, b) in string.chars().enumerate() {
            self.mode.draw_character(self.x + space * 8, self.y, b, Color16::White)
        }
    }

    pub fn pixel(&mut self) {
        self.mode.set_pixel(self.x, self.y, self.color);
    }
}

impl Write for VgaDisplay {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_string(s);
        Ok(())
    }
}

// -----------------
// Write/sys_write system call
//
// This is intended to be used with a high resolution graphics mode so it should be set before using
fn write(sys_arg: u8) -> u8 {
    let mode = Graphics640x480x16::new();
    mode.draw_character(0, 0, sys_arg as char, Color16::White);

    return 0;
}


