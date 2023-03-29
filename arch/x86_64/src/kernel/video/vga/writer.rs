use core::ptr::{read, read_volatile, write_volatile};
use crate::kernel::vga::{BUFFER_HEIGHT, BUFFER_WIDTH, VGA_ADDRESS};

pub struct VgaWriter {
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub height: u8,
    pub buffer_addr: *mut u8,
    pub addr: *mut u8,
}

impl VgaWriter {
    pub fn new() -> Self {
        return VgaWriter {
            x: 0,
            y: 2 as u8,
            width: BUFFER_WIDTH as u8,
            height: BUFFER_HEIGHT as u8,
            buffer_addr: VGA_ADDRESS as *mut u8,
            addr: VGA_ADDRESS as *mut u8,
        };
    }

    pub fn write_byte(&mut self, byte: u8) {
        if self.x == self.width || byte == b'\n' {
            self.new_line();
        } else {
            unsafe { write_volatile(self.addr.offset(self.x as isize + (self.x * self.y) as isize), byte); }

            self.x += 2;
        }
    }


    pub fn write_string(&mut self, string: &'static str) {
        for b in string.as_bytes() {
            self.write_byte(*b);
        }
    }

    pub fn new_line(&mut self) {
        for y in 1..self.height {
            for x in 0..self.width {
                let c = unsafe { read_volatile(self.addr.offset(x as isize).offset((x * y) as isize)) };
                unsafe { write_volatile(self.addr.offset(x as isize).offset((x * (y - 2)) as isize), c); }
            }
        }

        self.x = 0;
        self.y = 2;
    }

    pub fn clear_screen(&mut self) {
        for y in 0..self.height {
            for x in 0..self.height {
                self.write_string(" ");
            }
        }

        self.x = 0;
        self.y = self.height - 2;
    }
}