#![no_std]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DriverNames {
    Vga,
    Gop,
    ArmFb,
    None,
}
