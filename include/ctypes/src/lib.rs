#![no_std]

mod arm;
mod risc;
mod x86;
mod xtensa;

#[cfg(target_arch = "arm")]
pub use arm::*;

#[cfg(target_arch = "aarch64")]
pub use arm::*;

#[cfg(target_arch = "riscv32")]
pub use risc::*;

#[cfg(target_arch = "riscv64")]
pub use risc::*;

#[cfg(target_arch = "x86_64")]
pub use x86::*;

#[cfg(target_arch = "xtensa")]
pub use xtensa::*;

pub type c_int = i32;
pub type c_uint = u32;
pub type c_macro_int = usize;
