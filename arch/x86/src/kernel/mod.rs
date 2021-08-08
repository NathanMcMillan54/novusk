#[cfg(not(feature = "uefi_boot"))]
pub mod alloc;

pub(crate) mod cpu;
pub mod io;
pub(crate) mod kernel;
pub(crate) mod modules;
pub mod printk;
pub mod task;
pub(crate) mod x86_init;

#[path = "video/gop/mod.rs"]
pub(crate) mod gop;

#[path = "video/vga/mod.rs"]
pub(crate) mod vga;

