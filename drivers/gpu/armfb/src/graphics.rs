use core::any::Any;
use fb::{FbColor, FrameBufferGraphics};
use libcolor::hex_colors::HexColors;

pub const BLACK: usize = HexColors::Black as usize;
pub const WHITE: usize = HexColors::White as usize;
pub const LIGHT_GRAY: usize = HexColors::LightGray as usize;

#[no_mangle]
pub extern "C" fn graphics_write(x: usize, y: usize, color: usize, string: &str) {
    #[cfg(target_arch = "aarch64")]
    let armfb = crate::a64::A64Fb::new();

    #[cfg(target_arch = "arm")]
    let armfb = crate::a32::A32Fb::new();

    armfb.write_string(x, y, FbColor::new(color, 0, 0), string);
}

#[no_mangle]
pub extern "C" fn graphics_pixel(x: usize, y: usize, color: usize) {
    #[cfg(target_arch = "aarch64")]
    let armfb = crate::a64::A64Fb::new();

    #[cfg(target_arch = "arm")]
    let armfb = crate::a32::A32Fb::new();

    armfb.pixel(x, y, FbColor::new(color, 0, 0));
}

