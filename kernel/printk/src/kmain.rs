use core::fmt::Arguments;


pub struct KmainPrinter {
    pub x: usize,
    pub y: usize,
    pub color: usize,
}

impl KmainPrinter {
    pub fn new(print_x: usize, print_y: usize, print_color: usize) -> Self {
        return KmainPrinter { x: print_x, y: print_y, color: print_color };
    }

    pub fn write_fmt(&mut self, fmt: Arguments) {
        // unsafe { graphics_print(self.x, self.y, 15, fmt); }
        self.y = self.y + 1;
    }
}

pub fn _main_kernel_printk(args: Arguments) {
    let mut kmain_printer = KmainPrinter::new(0, 0, 15);

    kmain_printer.write_fmt(args);
}
