#![no_std]

#[cfg(feature = "esp32_board")]
pub extern crate esp32;

#[cfg(feature = "esp32_board")]
#[macro_use] extern crate esp32_hal;

pub mod drivers;
pub mod kernel;
