#![no_std]

pub mod info;

#[cfg(target_arch = "x86_64")]
pub extern crate x86;

#[cfg(target_arch = "x86")]
pub extern crate x86;

pub mod libs {
    pub use libcolor;
}

pub mod kernel {
    pub use kinfo;
    pub use printk;
}
