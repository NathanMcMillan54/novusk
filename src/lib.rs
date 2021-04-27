#![no_std]

#[cfg(target_arch = "aarch64")]
pub extern crate aarch64;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub extern crate x86;
