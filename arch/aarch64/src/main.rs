#![no_std]
#![no_main]
#![feature(asm, global_asm)]

#[macro_use] extern crate kerror;
#[macro_use] extern crate kinfo;

extern crate arm_lib;

pub mod boot;
pub mod drivers;
pub mod kernel;

#[cfg(feature = "board_rpi3")]
extern crate rpi3;
