#![no_std]

#[cfg(not(feature = "no_alloc"))]
#[macro_use] extern crate alloc;
#[macro_use] extern crate novuskinc;

#[cfg(not(feature = "no_oom"))]
pub mod oom;
#[cfg(not(feature = "no_panic"))]
pub mod panic;

#[cfg(not(feature = "inc_dev_drivers"))]
pub mod drivers;
