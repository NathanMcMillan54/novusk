#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
pub fn _panic(_info: &PanicInfo) -> ! {
    loop {  }
}
