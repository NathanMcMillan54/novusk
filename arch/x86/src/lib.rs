#![no_std]

use core::panic::PanicInfo;

pub mod boot;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}
