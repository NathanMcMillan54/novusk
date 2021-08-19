#![no_std]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;

#[cfg(target_arch = "x86_64")]
extern crate vgag;

pub mod color;
pub mod pixel;
pub mod print;

pub static mut DRIVER: DriverNames = DriverNames::None;


pub unsafe fn set_driver(name: DriverNames) {
    DRIVER = name;
}

pub unsafe fn gpu_graphics_init() {
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DriverNames {
    Vgag,
    Gop,
    ArmFb,
    None,
}
