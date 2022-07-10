#![no_std]

#[macro_use] extern crate novuskinc;

use core::panic::PanicInfo;

#[path = "../../../../kernel/panic.rs"]
pub mod panic;

pub fn armfb_init() {

}

module_init!(core_display_init, armfb_init);

pub fn armfb_end() {

}

module_end!(core_display_end, armfb_end);
