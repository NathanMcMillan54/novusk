#![no_std]

#[macro_use] extern crate novuskinc;
use novuskinc::core::prelude::*;

pub(crate) mod mailbox;

pub(crate) struct RaspberryPi {

}

fn raspberrypi_init() {

}

fn raspberrypi_end() {

}

#[no_mangle]
pub static DIF_FILE: &'static str = include_str!("../../../../arch/arm/src/include/dif/difs/rpi3b.dif");

module_init!(early_device_init, raspberrypi_init);
module_end!(early_device_end, raspberrypi_end);
