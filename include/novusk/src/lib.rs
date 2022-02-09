#![no_std]
#![allow(warnings)]

pub mod core;
pub mod fb;
pub mod mb;
pub mod module;
pub mod syscalls;
pub mod serial;

use ::core::panic::PanicInfo;

#[cfg(not(feature = "library"))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {  }
}
