#![no_std]
#![allow(warnings)]
#![feature(const_fn_trait_bound, const_mut_refs, panic_info_message)]

pub mod console;
pub mod core;
pub mod drivers;
pub mod elf;
pub mod fb;
pub mod irq;
pub mod kernel;
pub mod keyboard;
pub mod led;
pub mod mb;
pub mod module;
pub mod prelude;
pub mod syscalls;
pub mod serial;
pub mod timer;

use ::core::panic::PanicInfo;

#[cfg(not(feature = "library"))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {  }
}

#[cfg(not(target_os = "novusk"))]
compile_error!("Novuskinc is meant to only be used for Novusk based OSes");
