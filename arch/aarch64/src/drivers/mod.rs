#[cfg(feature = "board_virt")]
pub mod virt;

pub mod device;

pub use uefi_kd::init::uefi_init;