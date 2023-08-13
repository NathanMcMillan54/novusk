#![no_std]

#[macro_use] extern crate novuskinc;
#[macro_use] extern crate tock_registers;

use tm4c123x_hal::Peripherals;
use tm4c123x_hal::prelude::*;
use tm4c123x_hal::sysctl::{CrystalFrequency, Divider, Oscillator, PllOutputFrequency, Sysctl, SystemClock};

pub mod interrupts;

#[cfg(any(feature = "s6965", feature = "s811"))]
pub mod lm3s;

pub unsafe fn setup_sys_clock() {
    let peripherals = Peripherals::steal();
    let mut sysctl = peripherals.SYSCTL.constrain();

    sysctl.clock_setup.oscillator = Oscillator::Main(
        CrystalFrequency::_16mhz,
        SystemClock::UseOscillator(Divider::_2),
    );

    let clocks = sysctl.clock_setup.freeze();
}
