#![no_std]

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "x86_64")]
pub mod x64;

#[cfg(target_arch = "x86")]
pub mod x86;

pub mod arch;
pub mod cpu;
