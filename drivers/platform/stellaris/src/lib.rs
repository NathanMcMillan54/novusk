#![no_std]
#![feature(core_intrinsics)]
#![feature(asm)]

#[macro_use] extern crate novuskinc;

use core::panic::PanicInfo;
use cortex_m_semihosting::hprintln;

#[path = "dif.rs"]
pub mod dif;

pub(crate) mod common;
pub(crate) mod s6965;
pub(crate) mod s811;

fn stellaris_init() {
    common::stellaris_board_init();

    if dif::DIF_FILE[0] == "Stellaris LM3S6965" {
        s6965::lm3s6965_init();
    }
}


fn stellaris_end() {

}

module_init!(early_device_init, stellaris_init);
module_end!(early_device_end, stellaris_end);
