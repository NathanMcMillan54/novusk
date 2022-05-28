#![no_std]
#![feature(asm, global_asm)]
#![feature(panic_info_message)]
#![allow(warnings)]

#[macro_use] extern crate alloc;
pub(crate) extern crate nmallocator;
#[macro_use] extern crate novuskinc;
//pub(crate) extern crate arm;

/*#[cfg(feature = "bcm_soc")]
pub(crate) extern crate bcm;*/

#[path = "../aarch64.rs"]
pub mod arch;

pub use arch::*;
