#![no_std]
#![no_main]
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

#[no_mangle]
pub extern "C" fn application_type() -> libn::libnu::ktypes::ApplicationType { return libn::libnu::ktypes::ApplicationType::None; }

#[no_mangle]
pub extern "C" fn main_color() -> &'static str { return "cyan"; }

#[no_mangle]
pub extern "C" fn initramfs() -> bool { return false; }

#[no_mangle]
pub extern "C" fn initramfs_main() { return; }
