use crate::libx::libcolor::Color4Bit;
use super::early_vga::{Vga};

/// VGA text/graphics mode
pub const VGA: u8 = 0;
/// Graphic Output Protocol (UEFI)
pub const GOP: u8 = 1;
/// From Frame Buffer information provided by a bootloader
pub const BOOT_FRAMEBUFFER: u8 = 2;

pub unsafe fn set_video_driver(driver: u8) {
    if driver == VGA {
        static mut _VGA: Vga = Vga;
    }
}
