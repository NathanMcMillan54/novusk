#[cfg(target_arch = "x86_64")]
pub mod x64;

#[cfg(target_arch = "x86_64")]
pub use x64 as arch;

pub use arch::*;
