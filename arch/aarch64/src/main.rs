#![no_std]
#![no_main]
#![feature(asm, global_asm)]

extern crate novusk;

extern crate arm_lib;

use libn::libnu::ktypes::ApplicationType;

pub mod boot;
pub mod drivers;
pub mod kernel;

#[cfg(feature = "board_rpi3")]
extern crate rpi3;
