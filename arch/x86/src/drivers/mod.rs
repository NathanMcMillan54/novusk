pub mod amd;
pub mod device;
pub mod intel;

pub use uefi_kd::{print::print_uefi_init, init::uefi_init};
