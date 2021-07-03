pub(crate) mod amd;
pub(crate) mod drivers;
pub(crate) use drivers::Drivers;
pub(crate) mod grub;
pub(crate) mod intel;
pub(crate) mod ix86;
pub mod ps2;
pub mod vga;

// x86_64 drivers
// --------------
#[cfg(target_arch = "x86_64")]
pub(crate) mod x64_task;

#[cfg(target_arch = "x86_64")]
pub(crate) mod x64;


// --------------

// x86 drivers
// -----------
#[cfg(target_arch = "x86")]
mod x86;

// -----------
