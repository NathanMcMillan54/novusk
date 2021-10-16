#![no_std]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate printk;

#[cfg(target_arch = "x86_64")]
extern crate vgag;

pub mod color;
pub mod init;
pub mod pixel;
pub mod print;

use spin::Mutex;

pub struct GpuGraphics {
    pub driver_name: GpuDrivers,
    pub size: (usize, usize),
    pub write_fun: fn(usize, usize, usize, u8),
    pub pixel_fun: fn(usize, usize, usize),
}

lazy_static! {
    pub static ref GPUGRAPHICS: Mutex<GpuGraphics> = Mutex::new(GpuGraphics::new());
}

fn write(x: usize, y: usize, color: usize, byte: u8) {
    printk!("*write*");
    printk!("oh nvm");
}

fn pixel(x: usize, y: usize, color: usize) {
    printk!("*pixel*");
}

impl GpuGraphics {
    pub fn new() -> Self {
        return GpuGraphics {
            driver_name: Default::default(),
            size: (0, 0),
            write_fun: write,
            pixel_fun: pixel,
        };
    }

    pub fn set_driver(&mut self, driver: GpuDrivers) {
        self.driver_name = driver;
    }
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GpuDrivers {
    Vgag,
    Gop,
    ArmFb,
    None,
}

impl Default for GpuDrivers {
    fn default() -> Self {
        return GpuDrivers::None;
    }
}
