#![no_std]

extern crate arch;

extern crate pc_keyboard;

pub mod keyboard;
pub mod kinfo;
pub mod kprint;
pub mod user;

pub use user::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use arch::x86::x86lib::*;
