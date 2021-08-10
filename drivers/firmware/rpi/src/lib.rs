#![no_std]
#![feature(asm, llvm_asm)]

#[macro_use] extern crate printk;

pub mod board;
pub use board::check_board;
pub mod fb;
pub use fb::fb_init;
pub mod gpio;
pub use gpio::GPIO_BASE;

// This will probably be added to the Aarch64 kernel
pub mod rpi_mb;
