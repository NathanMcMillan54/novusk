#![no_std]
#![feature(asm)]

#[macro_use] extern crate novuskinc;
#[macro_use] extern crate tock_registers;

use core::ptr::write_volatile;
use soc::SocInfo;

#[path = "dif.rs"]
mod dif;

pub(crate) mod rpi3;

pub(crate) mod gpio;
pub(crate) mod led;
pub(crate) mod mailbox;
pub(crate) mod uart;

pub(crate) struct RaspberryPi {

}

unsafe fn raspberrypi_init() {
    gpio::gpio_init();
}

fn raspberrypi_end() {
    let act = led::RpiLed::new();

    act.blink(15000);
}

module_init!(early_device_init, raspberrypi_init);
module_end!(early_device_end, raspberrypi_end);

#[no_mangle]
pub extern "C" fn device_specific_irqs_init() {

}
