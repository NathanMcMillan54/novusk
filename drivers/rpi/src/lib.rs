#![no_std]
#![feature(asm, llvm_asm)]

#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;
#[macro_use] extern crate tock_registers;

pub mod bases;
pub use bases::*;
pub mod board;
pub use board::check_board;
pub mod fb;
pub use fb::fb_init;
pub mod gpio;
pub mod led;


// This will probably be added to the Aarch64 kernel
pub mod mb;

#[cfg(target_arch = "aarch64")]
pub fn aarch64_rpi_init(board: i8) {
    if board == 3 {

    } else if board == 4 {

    } else {  }
}

#[cfg(target_arch = "arm")]
pub fn arm_rpi_init(board: i8) {
    if board == 1 {

    } else if board == 2 {

    } else {  }
}
