#[cfg(target_arch = "x86_64")]
pub use vgag as graphics;

#[cfg(target_arch = "aarch64")]
pub use armfb as graphics;
