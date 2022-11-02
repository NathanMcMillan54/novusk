#![no_std]

#[macro_use] extern crate cfg_if;
#[macro_use] extern crate novuskinc;

use novuskinc::drivers::manager::DeviceDriverManager;
use novuskinc::kernel::types::KernelFunctionName;
use novuskinc::platform::{DRIVERS_FAILD, INVALID_DEVICE, UNKNOWN_ERROR};
use tm4c123x_hal::Peripherals;

cfg_if! {
    if #[cfg(feature = "irqchip")] {
        extern crate invic;
        extern crate irq;
    }
}

extern "C" {
    pub(crate) static mut DEVICE_DRIVERS: DeviceDriverManager;
}

pub mod irqs;
pub mod io;
pub mod led;


unsafe fn early_stellaris_init() -> u8 {

    0
}

define_kernel_function!(KernelFunctionName::early_device_init, -> u8, early_stellaris_init);

unsafe fn stellaris_init() -> u8 {

    0
}

define_kernel_function!(KernelFunctionName::device_init, -> u8, stellaris_init);
