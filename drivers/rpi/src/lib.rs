#![no_std]

#[macro_use] extern crate tock_registers;

pub mod board;
pub use board::RaspberryPi;
pub mod common;
pub use common::*;

pub mod rpi2;
pub mod rpi3;

pub use rpi2::Rpi2;
pub use rpi3::Rpi3;

#[cfg(feature = "rpi3")]
pub use rpi3::*;

#[cfg(target_arch = "aarch64")]
#[no_mangle]
#[export_name = "device_init"]
pub extern "C" fn rpi3_board_init() -> (Result<(), &'static str>, &'static str) {
    let mut pi = Rpi3::new();
    pi.init();

    return (Ok(()), "RPi 3");
}

#[cfg(target_arch = "arm")]
#[no_mangle]
#[export_name = "device_init"]
pub extern "C" fn rpi2_board_init() -> (Result<(), &'static str>, &'static str) {
    let mut pi = Rpi2::new();
    pi.init();

    return (Ok(()), "RPi 2");
}
