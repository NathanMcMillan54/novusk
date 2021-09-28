#![no_std]

#[macro_use] extern crate novuskinc;

use stm32f4xx_hal::pac::Peripherals;

pub mod io;

pub fn stm32f4_init() {
    let mut peripherals = Peripherals::take();

    if peripherals.is_none() {
        panic!("Can't find peripherals");
    } else { printk!("Got peripherals"); }
}
