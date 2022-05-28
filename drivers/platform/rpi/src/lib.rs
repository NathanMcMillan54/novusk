#![no_std]
#![allow(warnings)]

#[macro_use] extern crate novuskinc;

use core::panic::PanicInfo;

#[cfg(not(feature = "dev_selected"))]
compile_error!("RPi device not selected, add '--feature rpi(X)' to DEVICE_DRIVER");

pub mod dif;

#[cfg(feature = "rpi3")]
pub mod rpi3;

#[panic_handler]
fn _panic(_info: &PanicInfo) -> ! {
    loop {  }
}
