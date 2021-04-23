#![no_std]

extern crate drivers;
extern crate fs;
extern crate include;

#[macro_use]
extern crate kernel;

pub mod initramfs;
pub mod kmain;
pub mod version;
pub mod noinitramfs;

extern crate m1;
