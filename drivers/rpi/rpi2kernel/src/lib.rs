#![no_std]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;

pub(crate) mod drivers;
pub(crate) mod kernel;

pub mod init;
pub(crate) mod kmain;
