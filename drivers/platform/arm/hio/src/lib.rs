#![no_std]

pub mod driver;
pub mod io;
pub(crate) mod syscall;

pub struct HioDriver;
