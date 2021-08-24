#![no_std]
#![feature(asm, global_asm)]
// Lang
#![feature(abi_x86_interrupt, alloc_error_handler, const_mut_refs, panic_info_message)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;
#[macro_use] extern crate lazy_static;

#[cfg(feature = "uefi_boot")]
extern crate nkuefi;

use core::panic::PanicInfo;

pub mod boot;
pub mod include;
pub mod kernel;
pub mod libx;
pub mod mm;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    printk!("\nKernel panicked:");
    printk!("    Message: {:?}", _info.message().unwrap());
    printk!("    Location: {:?}", _info.location().unwrap());

    loop {  }
}
