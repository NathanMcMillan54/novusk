use kinfo::info::*;
use konfig::Konfig;
use spin::Mutex;
use vfs::RootDir;

lazy_static! {
    pub static ref KERNEL: Mutex<Kernel> = Mutex::new(Kernel);
}

pub struct Kernel;

impl Kernel {
    pub fn new() -> Self {
        return Kernel;
    }

    pub fn kernel_configs(&mut self) -> Konfig {
        return Konfig::new();
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn keyboard_driver(&mut self) {

    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn mouse_driver(&mut self) {

    }

    pub unsafe fn net_init(&mut self) {

    }

    pub fn get_root_dir(&mut self) -> RootDir {
        return RootDir::make();
    }
}
