#![no_std]

#[macro_use] extern crate cfg_if;
#[macro_use] extern crate novuskinc;
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


#[cfg(feature = "stellaris_6965")]
#[no_mangle]
pub unsafe extern "C" fn device_init() -> (Result<(), &'static str>, &'static str) {
    let cp = cortex_m::Peripherals::steal();

    let mut error = false;

    if unsafe { Peripherals::take().is_none() } {
        error = true;
    } else { error = false; }

    if error {
        return (Err("Cannot find peripherals"), "Stellaris 6965");
    } else { return (Ok(()), "Stellaris 6965"); }
}

#[no_mangle]
pub extern "C" fn rpi2_kernel_init() { }
