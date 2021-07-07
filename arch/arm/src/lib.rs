#![no_std]
#![feature(asm)]

use core::panic::PanicInfo;

#[panic_handler]
pub unsafe fn panic(_info: &PanicInfo) -> ! {
    loop { asm!("wfe"); }
}
