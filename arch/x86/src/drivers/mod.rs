pub mod amd;
pub mod drivers;
pub(crate) use drivers::Drivers;
pub mod grub;
pub mod intel;
pub mod ix86;
pub mod ps2;
pub mod vga;

// x86_64 drivers
// --------------
#[cfg(target_arch = "x86_64")]
pub mod x64_task;

#[cfg(target_arch = "x86_64")]
pub mod x64;


// --------------

// x86 drivers
// -----------
#[cfg(target_arch = "x86")]
mod x86;

// -----------
