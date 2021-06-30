#![no_std]
#![feature(asm, global_asm)]
// Lang
#![feature(alloc_error_handler, panic_info_message)]

extern crate alloc;
#[macro_use] extern crate kinfo;

#[cfg(feature = "uefi_boot")]
extern crate nkuefi;

use core::panic::PanicInfo;

pub mod boot;
pub mod drivers;
pub mod include;
pub mod kernel;
pub mod modules;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        x86_printk!("\nKernel panicked:");
        x86_printk!("   Message: {:?}", _info.message().unwrap());
        x86_printk!("   Location: {:?}", _info.location().unwrap());
    }
    loop {  }
}
