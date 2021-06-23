#[cfg(not(feature = "grub"))]
global_asm!(include_str!("header.S"));

pub mod boot;
pub mod bootloaders;
pub mod main;
