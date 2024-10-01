use core::fmt::{Arguments, Write};
use core::ptr::{read_volatile, write_volatile};
use crate::libx::libcolor::Color4Bit;

lazy_static! {
    pub static ref VGA_WRITER: spin::Mutex<VgaWriter> = spin::Mutex::new(VgaWriter::new());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    pub character: u8,
    pub color_code: u8,
}

impl ScreenChar {
    /// ``color1`` represents the background and ``color2`` represents the foreground,
    /// and ``c`` represents an 8 bit character.
    pub fn new(color1: Color4Bit, color2: Color4Bit, c: u8) -> Self {
        let color = color1 as u8 | (color2 as u8) << 4;

        return ScreenChar {
            character: c,
            color_code: color,
        };
    }
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;
pub const VGA_ADDRESS: usize = 0xb8000;

#[repr(transparent)]
pub struct VgaBuffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VgaWriter {
    pub x: usize,
    pub y: usize,
    pub background: Color4Bit,
    pub foreground: Color4Bit,
    pub buffer: &'static mut VgaBuffer,
}

impl VgaWriter {
    pub const fn new() -> Self {
        return VgaWriter {
            x: 0,
            y: BUFFER_HEIGHT - 2,
            background: Color4Bit::Black,
            foreground: Color4Bit::White,
            buffer: unsafe { &mut *(VGA_ADDRESS as *mut VgaBuffer) },
        };
    }

    pub fn write_byte(&mut self, b: u8) {
        let screen_char = ScreenChar::new(self.foreground, self.background, b);
        if b == b'\n' {
            self.scroll_screen();
            self.x = 0;
            return;
        }

        if self.x == BUFFER_WIDTH {
            self.x = 0;
            self.scroll_screen();
        }

        self.buffer.chars[self.y][self.x] = screen_char;

        self.x += 1;
    }

    pub fn read_byte(&mut self, x: usize, y: usize) -> u8 {
        self.buffer.chars[y][x].character
    }

    pub fn write_string(&mut self, string: &str) {
        for b in string.as_bytes() {
            self.write_byte(*b);
        }
    }

    pub fn scroll_screen(&mut self) {
        for y in 1..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                self.buffer.chars[y - 1][x] = self.buffer.chars[y][x];
            }
        }
    }
}

pub struct Vga;

impl Vga {
    pub fn writer_write_str(&self, string: &str) {
        VGA_WRITER.lock().write_string(string);
    }
}

impl Write for Vga {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.writer_write_str(s);
        Ok(())
    }
}
