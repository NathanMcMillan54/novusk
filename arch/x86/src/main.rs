#![no_std]
#![no_main]

#[macro_use] extern crate uefi;
#[macro_use] extern crate uefi_macros;

pub mod boot;

use core::fmt::Write;
use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}
