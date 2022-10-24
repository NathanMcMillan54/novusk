#![no_std]

pub(crate) extern crate bcm_soc;
extern crate nmallocator;
#[macro_use] extern crate novuskinc;
#[macro_use] extern crate tock_registers;

use novuskinc::drivers::manager::DeviceDriverManager;

pub mod common;
pub use common::*;

extern "C" {
    pub(crate) static mut DEVICE_DRIVERS: DeviceDriverManager;
}

#[macro_use]
#[path = "../../../kernel/irq.rs"]
pub mod irq;

#[cfg(feature = "rpi2")]
pub mod rpi2;

#[cfg(feature = "rpi3")]
pub mod rpi3;

#[cfg(feature = "rpi2")]
pub use rpi2::Rpi2;

#[cfg(feature = "rpi2")]
pub use rpi2::registers::*;

#[cfg(feature = "rpi3")]
pub use rpi3::*;

fn irq_init() {

}

define_dev_irq_init!(irq_init);
