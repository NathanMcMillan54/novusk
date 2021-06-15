#![no_std]

#[cfg(feature = "uefi_colors")]
pub mod uefi_colors;

#[cfg(feature = "vga_colors")]
pub mod vga_colors;
