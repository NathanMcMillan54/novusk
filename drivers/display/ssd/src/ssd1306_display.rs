use core::fmt::{Arguments, Write};
use novuskinc::fb::FrameBufferGraphics;
use crate::device_ssd_handler;
use crate::modes::DisplayMode;

pub struct Ssd1306Display {
    pub mode: DisplayMode,
    pub x: u8,
    pub y: u8,
}

impl Ssd1306Display {
    pub unsafe fn init() -> Self {
        device_ssd_handler(0, (0, 0), 0);
        return Ssd1306Display {
            mode: DisplayMode::KernelConsole,
            x: 0,
            y: 0,
        }
    }

    pub fn user_graphics_init(&mut self) {
        self.mode = DisplayMode::UserGraphics;
    }
}

impl FrameBufferGraphics for Ssd1306Display {
    fn graphics_write(&self, byte: u8, x: usize, y: usize) {
        unsafe { device_ssd_handler(self.mode.as_u8(), (x as u8, y as u8), byte) }
    }
}

impl Write for Ssd1306Display {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            self.x += 1;
            if b == &b'\n' {
                self.y += 1;
                self.x = 0;
            } else if self.x == 129 {
                self.x = 0;
            }

            self.graphics_write(*b, self.x as usize, self.y as usize);
        }

        Ok(())
    }
}
