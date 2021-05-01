#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod boot;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}