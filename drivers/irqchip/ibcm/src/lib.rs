#![no_std]
#![feature(panic_info_message)]

#[macro_use] extern crate asminc;
#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;

#[cfg(feature = "bcm2837")]
pub mod bcm2837;

#[path = "../../../../kernel/panic.rs"]
pub mod panic;
