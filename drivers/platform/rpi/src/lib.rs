#![no_std]
#![feature(asm)]

#[macro_use] extern crate asminc;
#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;
#[macro_use] extern crate tock_registers;

use core::fmt::Write;
use novuskinc::core::names::CoreFunctionNames;
use novuskinc::drivers::manager::DeviceDriverManager;
use novuskinc::prelude::KernelConsoleDriver;
use soc::info::SocInfo;
use crate::dif::DIF_FILE;

#[path = "dif.rs"]
mod dif;

pub(crate) mod rpi3;

pub(crate) mod gpio;
pub(crate) mod led;
pub(crate) mod mailbox;
pub(crate) mod uart;

extern "C" {
    pub(crate) static mut DEVICE_DRIVERS: DeviceDriverManager;
    pub(crate) static mut SOC_INFO: SocInfo;
}

unsafe fn raspberrypi_init() {
    rpi3::rpi3_init();
}

fn raspberrypi_end() {
    let act = led::RpiLed::new();

    act.blink(15000);
}

module_init!(early_device_init, raspberrypi_init);
module_end!(early_device_end, raspberrypi_end);

unsafe fn handle_irqs(irqn: i16) -> () {
    rpi3::irqs::rpi3_irq_handler(irqn);
}

define_core_function!(CoreFunctionNames::device_irq_handler, irqn: i16, -> (), handle_irqs);

unsafe fn device_irqs_init(_n: ()) -> () {
    rpi3::irqs::rpi3_enable_irqs();
}

define_core_function!(CoreFunctions::device_specific_irqs_init, _n: (), -> (), device_irqs_init);

#[no_mangle]
pub unsafe extern "C" fn set_platform_drivers() {
    rpi3::set_rpi3_drivers();
}
