use crate::VgaModes;

pub mod graphics;
pub mod text;

use graphics::{graphics_640x480_pixel, graphics_640x480_write};
use text::{text_80x25_pixel, text_80x25_write};

fn write(x: usize, y: usize, color: usize, string: &str) {    }
fn pixel(x: usize, y: usize, color: usize) {    }

pub struct VgaDisplay {
    pub write_fun: fn(usize, usize, usize, &str),
    pub pixel_fun: fn(usize, usize, usize),
}

impl VgaDisplay {
    pub fn new(vga_mode: VgaModes) -> Self {
        let mut write: Option<fn(usize, usize, usize, &str)>;
        let mut pixel: Option<fn(usize, usize, usize)>;

        if vga_mode == VgaModes::Text80x25 {
            write = Some(text_80x25_write);
            pixel = Some(text_80x25_pixel);
        } else if vga_mode == VgaModes::Graphics640x480 {
            write = Some(graphics_640x480_write);
            pixel = Some(graphics_640x480_pixel);
        } else {
            write = None;
            pixel = None;
        }

        return VgaDisplay {
            write_fun: write.unwrap(),
            pixel_fun: pixel.unwrap(),
        }
    }
}

