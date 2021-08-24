#![no_std]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;

#[cfg(target_arch = "x86_64")]
extern crate vgag;

pub mod color;
pub mod pixel;
pub mod print;

pub static mut DRIVER: DriverNames = DriverNames::None;

pub struct GpuGraphics;

impl GpuGraphics {
    pub fn new() -> Self {
        return GpuGraphics;
    }

    pub unsafe fn init(&mut self) {
        if DRIVER == DriverNames::Vgag {
            #[cfg(target_arch = "x86_64")]
            vgag::vgag_init();
        } else if DRIVER == DriverNames::ArmFb {
            #[cfg(target_arch = "aarch64")]
            armfb::armfb_init();

            #[cfg(target_arch = "arm")]
            armfb::armfb_init();
        } else {
            return;
        }
    }

    pub unsafe fn uninit(&mut self) {
        if DRIVER == DriverNames::Vgag {
            #[cfg(target_arch = "x86_64")]
            vgag::vgag_uninit();
        }
    }
}

pub unsafe fn set_driver(name: DriverNames) {
    DRIVER = name;
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DriverNames {
    Vgag,
    Gop,
    ArmFb,
    None,
}
