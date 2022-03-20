#![no_std]

#[macro_use] extern crate novuskinc;
use novuskinc::core::prelude::*;

#[path = "dif.rs"]
mod dif;

pub(crate) mod mailbox;

pub(crate) struct RaspberryPi {

}

fn raspberrypi_init() {

}

fn raspberrypi_end() {

}

module_init!(early_device_init, raspberrypi_init);
module_end!(early_device_end, raspberrypi_end);

#[no_mangle]
pub extern "C" fn device_specific_irqs_init() {

}
