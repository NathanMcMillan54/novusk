#![no_std]

#[macro_use] extern crate alloc;
pub(crate) extern crate irq;
#[macro_use] extern crate printk;
#[macro_use] extern crate novuskinc;

#[cfg(feature = "bcm2837")]
pub mod bcm2837;
