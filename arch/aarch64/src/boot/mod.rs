#[cfg(any(target_arch = "aarch64"))]
global_asm!(include_str!("init.S"));

pub mod main;
pub(crate) mod msg;
pub mod name;
