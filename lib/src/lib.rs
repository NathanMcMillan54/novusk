#![no_std]

extern crate arch;

pub mod kinfo;
pub mod kprint;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use arch::x86::lib::ui;
