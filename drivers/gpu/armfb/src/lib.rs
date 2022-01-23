#![no_std]

#[macro_use] extern crate novuskinc;

use core::panic::PanicInfo;

/* [macro_use] extern crate printk;

use fb::{Fb};

//#[cfg(target_arch = "aarch64")]
pub mod a64;

#[cfg(target_arch = "arm")]
pub(crate) mod a32;

pub mod graphics;

pub use graphics::*;

pub static mut ARMFB: ArmFb = ArmFb {
    first_init: true,
};

pub struct ArmFb {
    first_init: bool,
}

impl ArmFb {
    pub fn init(&mut self) {
        #[cfg(target_arch = "aarch64")]
        a64::a64_fb_init();
    }
}*/

pub fn armfb_init() {

}

module_init!(core_display_init, armfb_init);

pub fn armfb_end() {

}

module_end!(core_display_end, armfb_end);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {  }
}
