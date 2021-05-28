// https://github.com/reisub0/rust-raspi3-tutorial helps a lot

#![no_std]

extern crate rpi;

pub mod board;
pub mod boot;
pub mod linker;
pub mod power;
pub mod print;
pub mod setup;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}
