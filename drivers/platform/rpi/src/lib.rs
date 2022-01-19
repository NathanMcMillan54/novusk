#![no_std]

#[macro_use] extern crate novuskinc;
use novuskinc::core::prelude::*;

pub(crate) struct RaspberryPi {

}

fn raspberrypi_init() {

}

fn raspberrypi_end() {

}

module_init!(early_device_init, raspberrypi_init);
module_end!(early_device_end, raspberrypi_end);
