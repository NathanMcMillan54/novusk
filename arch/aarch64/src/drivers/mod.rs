#[cfg(feature = "board_virt")]
pub mod virt;

#[cfg(feature = "board_rpi3")]
pub mod rpi;

pub use uefi_kd::init::uefi_init;