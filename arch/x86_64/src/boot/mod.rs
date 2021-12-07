#[cfg(feature = "bootloader_rs")]
global_asm!(include_str!("header.S"));

pub(crate) mod boot;
pub(crate) mod loaders;
pub mod main;
