#![no_std]
#![feature(core_intrinsics)]
#![feature(asm)]

#[macro_use] extern crate novuskinc;

use core::arch::asm;
use core::panic::PanicInfo;
use cortex_m_semihosting::hprintln;

#[path = "dif.rs"]
pub mod dif;

pub(crate) mod common;
pub(crate) mod s6965;
pub(crate) mod s811;

fn stellaris_init() {
    common::stellaris_board_init();
}


fn stellaris_end() {

}

module_init!(early_device_init, stellaris_init);
module_end!(early_device_end, stellaris_end);
