#![no_std]


extern crate x86_64;
#[cfg(any(target_arch = "x86_64"))]
use x86_64::{include::asm::{hlt}};

mod kernel;

use core::panic::{PanicInfo};

#[no_mangle]
pub unsafe extern "C" fn kernel_init() -> ! {
    hlt()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}
