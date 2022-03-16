#![no_std]
#![feature(asm, global_asm)]
#![feature(alloc_error_handler, lang_items, panic_info_message)]

#[macro_use] extern crate cfg_if;
#[macro_use] extern crate cortex_m_semihosting;
#[macro_use] extern crate novuskinc;
pub(crate) extern crate rlibc;

#[path = "../arm.rs"]
pub mod arch;

pub use arch::*;

#[cfg(target_arch = "aarch64")]
pub mod bits64;

#[lang = "eh_personality"]
extern "C" fn eh_personality() { }
