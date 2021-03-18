#![no_std]
#![feature(asm)]

use core::panic::{PanicInfo};

#[no_mangle]
pub unsafe extern "C" fn kernel_init() -> ! {
    loop { asm!("hlt") }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}