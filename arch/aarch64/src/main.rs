#![no_std]
#![no_main]
#![feature(asm, global_asm)]

use core::panic::PanicInfo;

extern crate arm_lib;

pub mod boot;
pub mod drivers;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}