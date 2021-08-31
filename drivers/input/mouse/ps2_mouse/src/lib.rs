#![no_std]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;

pub mod device;

pub struct Ps2Mouse;

impl Ps2Mouse {
    pub fn new() -> Self {
        return Ps2Mouse;
    }
}

