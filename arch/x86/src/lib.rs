#![no_std]
#![feature(alloc_error_handler, asm, global_asm)]

extern crate alloc;
#[macro_use] extern crate kinfo;
extern crate nkuefi;

use core::panic::PanicInfo;

pub mod boot;
pub mod drivers;
pub mod include;
pub mod kernel;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}
