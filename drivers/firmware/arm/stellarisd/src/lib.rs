#![no_std]

#[macro_use] extern crate novuskinc;
use tm4c123x_hal::Peripherals;

pub mod led;

pub mod board {

}

pub fn stellars_init() {
    if Peripherals::take().is_none() {
        panic!("Can't find peripherals");
    }
}
