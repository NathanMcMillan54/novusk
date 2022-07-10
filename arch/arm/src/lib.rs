#![no_std]
#![feature(panic_info_message)]

extern crate nmallocator;
#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;

pub mod boot;
pub mod include;
pub mod kernel;
