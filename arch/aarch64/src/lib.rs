#![no_std]

#![feature(asm, global_asm)]

#[macro_use] extern crate kerror;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;
extern crate userspace;

#[macro_use] extern crate lazy_static;

extern crate novusk;
extern crate arm_lib;

pub mod boot;
pub mod drivers;
pub mod kernel;

#[cfg(feature = "android_os")]
extern crate android;

#[cfg(feature = "board_rpi3")]
extern crate rpi3;
