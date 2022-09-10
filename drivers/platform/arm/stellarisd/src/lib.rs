#![no_std]

#[macro_use] extern crate cfg_if;
#[macro_use] extern crate novuskinc;

use novuskinc::kernel::types::KernelFunctionName;
use novuskinc::platform::{DRIVERS_FAILD, INVALID_DEVICE, UNKNOWN_ERROR};
use tm4c123x_hal::Peripherals;
use device::Device;

cfg_if! {
    if #[cfg(feature = "irqchip")] {
        extern crate invic;
        extern crate irq;
    }
}

pub mod irqs;
pub mod io;
pub mod led;

pub struct Stellaris6965;

impl Stellaris6965 {
    pub fn new() -> Self {
        return Stellaris6965;
    }
}

unsafe fn early_stellaris_init() -> u8 {

    0
}

define_kernel_function!(KernelFunctionName::early_device_init, -> u8, early_stellaris_init);

unsafe fn stellaris_init() -> u8 {

    0
}

define_kernel_function!(KernelFunctionName::device_init, -> u8, stellaris_init);
