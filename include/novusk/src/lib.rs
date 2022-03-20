#![no_std]
#![allow(warnings)]
#![feature(const_fn_trait_bound, const_mut_refs, panic_info_message)]

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
    cortex_m_semihosting::hprintln!("Panic: {} {}:{}", info.message().unwrap(), info.location().unwrap().file(), info.location().unwrap().line());

    loop {  }
}

#[cfg(not(target_os = "novusk"))]
compile_error!("Novuskinc is meant to only be used for Novusk based OSes");
