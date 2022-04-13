use core::fmt::{Arguments, Write};
use libcolor::{ColorCode, bits16::Color16};
use super::{ScreenChar, VgaG, VgaMode};
use volatile::Volatile;

pub struct Vga80x25Buffer {
    chars: [[Volatile<ScreenChar>; Vga80x25::WIDTH]; Vga80x25::HEIGHT],
}

pub struct Vga80x25 {
    pub col: usize,
    pub color: ColorCode,
    pub buffer: &'static mut Vga80x25Buffer,
    pub size: (u16, u16),
}

impl Vga80x25 {
    pub fn new() -> Self {
        return Vga80x25 {
            col: 0,
            color: ColorCode::new(Color16::White as u8, Color16::Black as u8),
            buffer: unsafe { &mut *(Vga80x25::BUFFER_ADDRESS as *mut Vga80x25Buffer) },
            size: (80, 25),
        }
    }

    fn new_line(&mut self) {
        for row in 1..Vga80x25::HEIGHT {
            for col in 0..Vga80x25::WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(Vga80x25::HEIGHT - 1);
        self.col = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            character: b' ',
            color: self.color,
        };

        for col in 0..Vga80x25::WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl VgaMode for Vga80x25 {
    const WIDTH: usize = 80;
    const HEIGHT: usize = 25;
    const BUFFER_ADDRESS: usize = 0xb8000;

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col >= Vga80x25::WIDTH {
                    self.new_line();
                }

                let row = Vga80x25::HEIGHT - 1;

                self.buffer.chars[row][self.col].write(ScreenChar {
                    character: byte,
                    color: self.color,
                });
                self.col += 1;
            }
        }
    }
}

impl Write for Vga80x25 {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            self.write_byte(*b);
        }

        Ok(())
    }
}
