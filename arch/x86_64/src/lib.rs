#![no_std]
#![feature(asm)]

use core::panic::PanicInfo;

#[panic_handler]
unsafe fn _panic(_info: &PanicInfo) -> ! {
    loop { asm!("hlt"); }
}
