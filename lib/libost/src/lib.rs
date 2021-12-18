#![no_std]

#[macro_use] extern crate alloc;

#[cfg(target_arch = "aarch64")]
pub(crate) extern crate armfb as graphics;

#[cfg(target_arch = "x86_64")]
pub(crate) extern crate vgag as graphics;

#[cfg(target_arch = "arm")]
pub(crate) mod graphics {
    pub fn graphics_pixel(x: usize, y: usize, color: usize) {

    }

    pub fn graphics_write(x: usize, y: usize, color: usize, string: &str) {

    }
}

pub mod desktop;
pub mod traits;
