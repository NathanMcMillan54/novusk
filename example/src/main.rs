#![no_std]
#![no_main]

#[macro_use]
extern crate novusk;

use core::panic::PanicInfo;

mod required;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    loop {    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}

