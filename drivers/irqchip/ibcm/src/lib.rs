#![no_std]

#[macro_use] extern crate asminc;
#[macro_use] extern crate novuskinc;

pub use bcm::SOC_INFO;

#[cfg(feature = "bcm2837")]
pub mod bcm2837;
