pub mod amd;
pub mod drivers;
pub mod grub;
pub mod intel;
pub mod ix86;
pub mod vga;

#[cfg(target_arch = "x86_64")]
pub mod x64_task;

#[cfg(target_arch = "x86_64")]
pub mod x64;
