#![no_std]
#![feature(asm)]

#[macro_use] extern crate novuskinc;
#[macro_use] extern crate tock_registers;

use core::fmt::Write;
use soc::SocInfo;
use novuskinc::console::KernelConsoleDriver;
use novuskinc::drivers::manager::DeviceDriverManager;
use crate::rpi3::Rpi3;

#[path = "dif.rs"]
mod dif;

pub(crate) mod rpi3;

pub(crate) mod gpio;
pub(crate) mod led;
pub(crate) mod mailbox;
pub(crate) mod uart;

extern "C" {
    static mut DEVICE_DRIVERS: DeviceDriverManager;
}

unsafe fn raspberrypi_init() {
    /* match dif::DIF_FILE[0] {
        "RaspberryPi 3B" => rpi3::rpi3_init(),
        _ => panic!("This driver is meant for RaspberryPi boards not {}", dif::DIF_FILE[0]),
    } */
    rpi3::rpi3_init();
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
