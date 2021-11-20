#![no_std]

// #[cfg(target_arch = "xtensa")]
pub mod xtensa;

#[cfg(feature = "esp32")]
pub fn get_board() -> xtensa::esp32::Esp32 {
    return xtensa::esp32::Esp32::new();
}
