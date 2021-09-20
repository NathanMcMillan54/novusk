#![no_std]

#[macro_use] extern crate xtensa_lx_rt;

#[cfg(feature = "esp32")]
#[macro_use] extern crate esp32_hal;

pub mod boot;
pub mod kernel;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}
