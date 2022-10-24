#![no_std]

#[macro_use] extern crate novuskinc;
#[macro_use] extern crate tock_registers;

#[cfg(feature = "bcm2837")]
pub mod bcm2837;

#[cfg(feature = "bcm2837")]
pub use bcm2837::*;