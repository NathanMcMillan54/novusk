#![no_std]

extern crate include;

pub mod printk;

extern "C" {
    pub fn die() -> !;
}
