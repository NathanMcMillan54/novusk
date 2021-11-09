#![no_std]
#![feature(asm, global_asm)]
// Lang
#![feature(abi_x86_interrupt, alloc_error_handler, const_mut_refs, panic_info_message)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate novuskinc;

#[cfg(feature = "uefi_boot")]
extern crate nkuefi;

use core::panic::PanicInfo;
pub use x86_64::instructions::port::Port;

pub mod boot;
pub mod include;
pub mod kernel;
pub mod libx;
pub mod mm;
pub mod net;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    use include::asm::{cli, hlt};
    use time::cpu::CPU_TIME;

    printk!("\nKernel panicked:\n");
    printk!("    Message: {:?}\n", _info.message().unwrap());
    printk!("    Location: {:?}\n", _info.location().unwrap());


    unsafe {
        printk!("CPU time: {}\n", CPU_TIME as f64 / 1000000000.0);
        hlt();
    }
}
