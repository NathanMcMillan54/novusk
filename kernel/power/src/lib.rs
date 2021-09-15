#![no_std]

#[macro_use] extern crate printk;

pub mod reboot;

pub struct Power;

impl Power {
    pub fn new() -> Self {
        return Power;
    }
}
