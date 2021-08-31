use gpu::GpuGraphics;
use konfig::Konfig;
use printk::console::KernelConsole;
use spin::Mutex;

lazy_static! {
    pub static ref KERNEL: Mutex<Kernel> = Mutex::new(Kernel);
}

pub struct Kernel;

impl Kernel {
    pub fn new() -> Self {
        return Kernel;
    }

    pub fn gpu_graphics(&mut self) -> GpuGraphics {
        return GpuGraphics::new();
    }

    pub fn kernel_console(&mut self) -> KernelConsole {
        return KernelConsole::new();
    }

    pub fn kernel_configs(&mut self) -> Konfig {
        return Konfig::new();
    }

    #[cfg(target_arch = "x86_64")]
    pub fn keyboard_driver(&mut self) -> pc_keyboard::PcKeyboard {
        return pc_keyboard::PcKeyboard::new();
    }

    #[cfg(target_arch = "x86_64")]
    pub fn mouse_driver(&mut self) -> ps2_mouse::Ps2Mouse {
        return ps2_mouse::Ps2Mouse::new();
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn keyboard_driver(&mut self) {

    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn mouse_driver(&mut self) {

    }
}
