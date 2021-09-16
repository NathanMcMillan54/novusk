#![no_std]
#![feature(asm, llvm_asm)]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;
#[macro_use] extern crate tock_registers;

pub mod bases;
pub use bases::*;
pub mod board;
pub use board::check_board;
pub mod debug;
pub mod fb;
pub use fb::RpiFb;
pub mod gpio;
pub use gpio::*;
pub mod led;
pub use led::RpiLed;


// This will probably be added to the Aarch64 kernel
pub mod mb;

pub fn aarch64_rpi_init(board: i8) {
    if board == 3 {
        let mut led = led::RpiLed::new();
        led.init();
        kinfo!("ACT LED initialized");
    }
}

pub fn arm_rpi_init(board: i8) {
    if board == 1 {

    } else if board == 2 {

    } else {  }
}
