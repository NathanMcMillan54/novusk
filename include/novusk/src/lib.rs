#![no_std]
#![allow(warnings)]
#![feature(const_fn_trait_bound, const_mut_refs)]

pub mod core;
pub mod elf;
pub mod fb;
pub mod keyboard;
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
