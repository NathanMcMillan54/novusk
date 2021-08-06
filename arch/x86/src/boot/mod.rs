#[cfg(not(feature = "grub"))]
global_asm!(include_str!("header.S"));

/*#[cfg(feature = "grub")]
global_asm!(include_str!("grub.asm"));*/

pub(crate) mod boot;
pub mod main;
