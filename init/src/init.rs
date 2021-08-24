use gpu::GpuGraphics;
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
}
