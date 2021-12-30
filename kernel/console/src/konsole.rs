use crate::Console;

pub struct KernelConsole {
    pub console_size: (usize, usize),
    pub column: usize,
    pub row: usize,
    pub color: usize,
}

impl KernelConsole {
    pub fn new(size: (usize, usize), console_color: usize) -> Self {
        return KernelConsole {
            console_size: size,
            column: 0,
            row: 0,
            color: console_color,
        };
    }

    pub fn display_kernel_console(&self) {
        for y in 0..self.console_size.1 {
            for x in 0..self.console_size.0 {
                // Put pixel(self.color);
            }
        }
    }
}

impl Console for KernelConsole {
    fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.row += 1;
        } else { self.column += 1; }
    }
}
