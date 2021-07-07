#![no_std]

#[cfg(target_arch = "x86_64")]
pub extern crate x86;

#[cfg(target_arch = "x86")]
pub extern crate x86;

#[cfg(target_arch = "aarch64")]
pub extern crate aarch64;

#[cfg(target_arch = "arm")]
pub extern crate arm32;

pub mod libs {
    pub use libcolor;
}

pub mod kernel {
    pub use kinfo;
    pub use printk;
}
