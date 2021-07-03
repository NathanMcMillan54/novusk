#[cfg(not(feature = "uefi_boot"))]
pub mod alloc;

pub(crate) mod cpu;
pub mod io;
pub(crate) mod kernel;
pub(crate) mod modules;
pub mod printk;
pub(crate) mod x86_init;
pub mod x86_start;
