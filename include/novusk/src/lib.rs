#![no_std]
#![allow(warnings)]

pub mod core;
pub mod module;
pub mod syscalls;

use ::core::panic::PanicInfo;

#[cfg(not(feature = "library"))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {  }
}
