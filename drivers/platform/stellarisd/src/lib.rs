#![no_std]

#[macro_use] extern crate cfg_if;
#[macro_use] extern crate novuskinc;

use novuskinc::drivers::manager::DeviceDriverManager;
use novuskinc::kernel::types::KernelFunctionName;
use novuskinc::platform::{DRIVERS_FAILD, INVALID_DEVICE, UNKNOWN_ERROR};
use tm4c123x_hal::Peripherals;

// Driver features
cfg_if! {
    if #[cfg(feature = "irqchip")] {
        pub extern crate invic;
        pub extern crate irq;
    }
}
/*#[cfg(feature = "kernel_init")]
pub extern crate init;*/

extern "C" {
    pub(crate) static mut DEVICE_DRIVERS: DeviceDriverManager;
}

pub mod common;
pub mod irqs;
pub mod io;
pub mod led;

#[cfg(feature = "stellaris_6965")]
pub mod s6965;

unsafe fn early_stellaris_init() -> u8 {
    common::common_early_stellaris_init();

    0
}

define_kernel_function!(KernelFunctionName::early_device_init, -> u8, early_stellaris_init);

unsafe fn stellaris_init() -> u8 {

    0
}

define_kernel_function!(KernelFunctionName::device_init, -> u8, stellaris_init);
