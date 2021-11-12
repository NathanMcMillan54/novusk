#![no_std]
#![feature(asm, llvm_asm)]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;
#[macro_use] extern crate tock_registers;

pub use bases::*;
pub use board::check_board;
pub use fb::RpiFb;
pub use gpio::*;

pub mod bases;
pub mod board;
pub mod debug;
pub mod fb;
pub mod gpio;
pub mod rpi3;
use rpi3::Rpi3;
use core::marker::PhantomData;


// This will probably be added to the Aarch64 kernel
pub mod mb;

pub fn aarch64_rpi_init(board: i8) {
    if board == 3 {
        let mut rpi = Rpi3::new();
        rpi.init();
    } else if board == 4 {
        // let mut rpi = Rpi4::new();
        // rpi.init();
    }
}

pub fn arm_rpi_init(board: i8) {
    if board == 1 {

    } else if board == 2 {

    } else {  }
}
