#![no_std]
#![no_main]
#![feature(global_asm, llvm_asm)]

#[macro_use] extern crate uefi;
#[macro_use] extern crate uefi_macros;
#[macro_use] extern crate uefi_services;

#[cfg(target_arch = "x86")]
extern crate i686;

pub mod boot;
pub mod drivers;
pub mod include;

use core::panic::PanicInfo;


/*
There shouldn't be a panic handler because uefi-services already has one
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}
*/
