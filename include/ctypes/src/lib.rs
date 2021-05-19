#![no_std]

pub type c_int = i32;
pub type macro_int = usize;

#[cfg(target_arch = "aarch64")]
pub type c_char = u8;

#[cfg(target_arch = "x86_64")]
pub type c_char = i8;

#[cfg(target_arch = "x86")]
pub type c_char = i8;
