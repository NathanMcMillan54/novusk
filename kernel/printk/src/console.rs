use core::fmt::Arguments;

static mut WRITER_Y: usize = 0;

pub struct KernelConsole {
    pub x: usize,
    pub y: usize,
}

impl KernelConsole {
    pub fn new() -> Self {
        return KernelConsole { x: 0, y: 0 }
    }

    pub unsafe fn init(&mut self) {
        super::KMAIN_PRINT = true;
    }

    pub unsafe fn uninit(&mut self) {
        super::KMAIN_PRINT = false;
        WRITER_Y = 0;

        self.y = WRITER_Y;
        self.x = 0;
    }

    pub fn set_xy(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn write_fmt(&mut self, fmt: Arguments) {
        extern "C" {
            fn _kernel_main_print(args: Arguments);
        }

        unsafe {
            _kernel_main_print(fmt);
        }
    }
}
