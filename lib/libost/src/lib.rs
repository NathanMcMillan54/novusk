#![no_std]

#[macro_use] extern crate alloc;

#[cfg(target_arch = "arm")]
pub(crate) mod graphics {
    pub fn graphics_pixel(x: usize, y: usize, color: usize) {

    }

    pub fn graphics_write(x: usize, y: usize, color: usize, string: &str) {

    }
}

pub mod desktop;
pub mod traits;
