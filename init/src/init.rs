use gpu::GpuGraphics;
use kinfo::info::*;
use konfig::Konfig;
use novuskinc::fs::RootDir;
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
        extern "C" {
            fn ethernet_init();
            fn wireless_init();
        }

        //if NETWORK_ETHERNET == true {
            ethernet_init();
        //} else if NETWORK_WIRELESS == true {
            wireless_init();
        //}

        /* if NETWORK_ETHERNET == false && NETWORK_WIRELESS == false {
            printk!("There are no networking drivers available for this architecture or device");
        } */
    }

    pub fn get_root_dir(&mut self) -> RootDir {
        return RootDir::make();
    }
}
