#![no_std]

extern crate arch;
use arch::ARCH;

pub mod kinfo;
pub mod kprint;
pub mod user;
pub use user::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use arch::x86::lib::*;
