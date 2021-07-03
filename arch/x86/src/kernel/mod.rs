#[cfg(not(feature = "uefi_boot"))]
pub mod alloc;

pub mod cpu;
pub mod io;
pub mod kernel;
pub mod modules;
pub mod printk;
pub mod x86_init;
pub mod x86_start;
