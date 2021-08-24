use core::fmt::Arguments;
use spin::Mutex;


lazy_static! {
    pub static ref KERNEL_CONSOLE: Mutex<KernelConsole> = Mutex::new(KernelConsole::new());
}

static mut WRITER_Y: usize = 0;

pub struct KernelConsole {
    pub x: usize,
    pub y: usize,
}

impl KernelConsole {
    pub fn new() -> Self {
        return KernelConsole { x: 0, y: 0 }
    }

    pub unsafe  fn init(&mut self) {
        super::KMAIN_PRINT = true;
    }

    pub fn set_xy(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn write_fmt(&mut self, fmt: Arguments) {
        extern "C" {
            fn _kernel_main_print(x: usize, y: usize, args: Arguments);
        }

        unsafe {
            WRITER_Y = WRITER_Y + 12;
            _kernel_main_print(self.x, WRITER_Y, fmt);
        }
    }
}
