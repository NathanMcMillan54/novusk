#![no_std]
#![allow(warnings)]
#![feature(panic_info_message)]

#[macro_use] extern crate novuskinc;

use core::panic::PanicInfo;
use novuskinc::drivers::manager::DeviceDriverManager;

#[cfg(not(feature = "dev_selected"))]
compile_error!("RPi device not selected, add '--feature rpi(X)' to DEVICE_DRIVER");

extern "C" {
    pub(crate) static mut DEVICE_DRIVERS: DeviceDriverManager;
}

pub mod dif;

#[path = "../../../../kernel/panic.rs"]
pub mod panic;

#[cfg(feature = "rpi3")]
pub mod rpi3;
