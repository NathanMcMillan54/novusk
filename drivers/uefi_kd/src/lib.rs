#![no_std]

#[macro_use] extern crate kerror;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;
extern crate gop;
extern crate gpu;

#[macro_use] extern crate alloc;
#[macro_use] extern crate uefi;

pub mod init;
pub mod print;
pub mod text;

pub static mut UEFI_MAJOR_VERSION: u16 = 0;
pub static mut UEFI_MINOR_VERSION: u16 = 0;
