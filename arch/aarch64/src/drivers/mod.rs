#[cfg(feature = "board_virt")]
pub mod virt;

pub use uefi_kd::init::uefi_init;