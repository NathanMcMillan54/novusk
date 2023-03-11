#![no_std]
#![feature(panic_info_message)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate novuskinc;

pub(crate) extern crate nmallocator;

#[cfg(feature = "qemu_virt")]
pub(crate) extern crate virt;

use core::panic::PanicInfo;

mod boot;
pub mod include;
//pub mod kernel;
//pub mod liba64;
pub mod mm;
//mod net;

// build.rs generates dif.rs, it should be ignored from git
pub(crate) mod dif;

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! { loop{} }
