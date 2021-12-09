#![no_std]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;

pub use rpi::rpi2::{mb::Rpi2Mb, uart::Rpi2Uart};

pub(crate) mod drivers;
pub mod init;
pub(crate) mod kmain;
