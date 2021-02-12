#![no_std]

#[cfg(any(target_arch = "aarch64"))]
pub extern crate aarch64;
#[cfg(any(target_arch = "aarch64"))]
pub use aarch64::ARCH;

#[cfg(any(target_arch = "arm", aarch64))]
pub extern crate arm;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub extern crate x86;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use x86::ARCH;