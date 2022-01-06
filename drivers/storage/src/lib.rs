#![no_std]

#[macro_use] extern crate novuskinc;

pub mod init;
pub mod storage_dev;

pub use init::storage_init;
pub use storage_dev::*;
