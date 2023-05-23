#![no_std]

#[macro_use] extern crate novuskinc;

pub mod interrupts;

#[cfg(any(feature = "s6965", feature = "s811"))]
pub mod lm3s;
