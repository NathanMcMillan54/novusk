pub mod device;

#[cfg(feature = "uefi_rpi3")]
pub mod rpi3;

pub use uefi_kd::init::uefi_init;