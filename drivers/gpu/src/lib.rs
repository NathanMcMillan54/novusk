#![no_std]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;

#[cfg(target_arch = "x86_64")]
pub mod vgag;


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DriverNames {
    Vga,
    Gop,
    ArmFb,
    None,
}
