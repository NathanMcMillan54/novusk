#![no_std]
#![feature(asm)]

use core::panic::PanicInfo;

pub mod boot;

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    loop {  }
}
