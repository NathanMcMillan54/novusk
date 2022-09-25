use crate::Console;
use core::str::from_utf8;

pub struct KernelConsole {
    pub console_size: (usize, usize),
    pub column: usize,
    pub row: usize,
    pub color: usize,
    // pub gpug: GpuGraphics,
}

impl KernelConsole {
    pub fn new(size: (usize, usize), console_color: usize) -> Self {
        return KernelConsole {
            console_size: size,
            column: 0,
            row: 0,
            color: console_color,
            // gpug: GpuGraphics::new(),
        };
    }

    pub fn display_kernel_console(&self) {
        for y in 0..self.console_size.1 {
            for x in 0..self.console_size.0 {
                // self.gpug.draw_pixel(x, y, self.color);
            }
        }
    }
}

impl Console for KernelConsole {
    fn write_byte(&mut self, byte: u8) {
        #[cfg(target_arch = "x86_64")]
        let white = 15;

        #[cfg(target_arch = "aarch64")]
        let white = 0xffffff;

        #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
        let white = 1;

        if byte == b'\n' {
            self.row += 1;
        } else {
            self.column += 1;
            // unsafe { self.gpug.write_string(self.column, self.row, white, from_utf8(&[byte]).unwrap()); }
        }
    }
}
