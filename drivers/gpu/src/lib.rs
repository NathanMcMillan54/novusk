#![no_std]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;

pub mod info;
pub mod init;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DriverNames {
    Vga,
    Gop,
    RpiFb,
    None,
}
