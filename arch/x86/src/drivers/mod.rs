pub mod amd;
pub mod grub;
pub mod ix86;
pub mod vga;

#[cfg(target_arch = "x86_64")]
pub mod x64_task;
