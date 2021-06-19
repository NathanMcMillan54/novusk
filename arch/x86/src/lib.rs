#![no_std]
#![feature(asm, global_asm)]

extern crate nkuefi;

use core::panic::PanicInfo;

pub mod boot;
pub mod drivers;
pub mod include;
pub mod kernel;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}
