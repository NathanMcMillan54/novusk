#![no_std]

#[macro_use] extern crate alloc;

#[cfg(target_arch = "x86_64")]
pub(crate) extern crate vgag as graphics;

pub mod desktop;
pub mod traits;
