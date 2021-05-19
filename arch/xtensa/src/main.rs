#![no_main]
#![no_std]

use core::panic::PanicInfo;

extern crate xtensa_novusk;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}
