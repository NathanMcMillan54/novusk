use core::{fmt::Write, ptr::write_volatile};

const VGA_BASE_ADDR: usize = 0xb8000;
const WIDTH: usize = 80;
const HEIGHT: usize = 25;

pub struct EarlyVga {
    pub x_pos: usize,
    pub y_pos: usize,
    pub address: *mut u8,
}

impl EarlyVga {
    pub fn new() -> Self {
        return EarlyVga {
            x_pos: 0,
            y_pos: 0,
            address: VGA_BASE_ADDR as *mut u8,
        };
    }

    pub fn write_byte(&mut self, b: u8) {
        unsafe {
            self.address = (VGA_BASE_ADDR as *mut u8).offset((self.x_pos * 2 + self.y_pos * 2) as isize);

            match b {
                b' ' => {},
                b'\n' => {
                    self.y_pos += 80;
                    self.x_pos = 0;
                },
                _ => write_volatile(self.address, b),
            };

            self.x_pos += 1;
            if self.x_pos == 25 {
                self.x_pos = 0;
                self.y_pos += 80;
            }
        }
    }

    pub fn delete(&mut self) {
        unsafe {
            self.x_pos -= 1;
            self.address = (VGA_BASE_ADDR as *mut u8).offset((self.x_pos * 2 + self.y_pos * 2) as isize);
            self.write_byte(b' ');
        }
    }
}

impl Write for EarlyVga {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            self.write_byte(*b);
        }

        Ok(())
    }
}
