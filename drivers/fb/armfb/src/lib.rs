#![no_std]

#[macro_use] extern crate novuskinc;

use core::cell::Cell;
use core::fmt::{Arguments, Write};
use novuskinc::console::KernelConsoleDriver;
use novuskinc::drivers::{Driver, DriverResult};
use novuskinc::drivers::names::FRAME_BUFFER;
use novuskinc::fb::{Color, FbInfo, FrameBufferGraphics};
use novuskinc::keyboard::KeyboardInput;
use novuskinc::led::Led;
use novuskinc::prelude::{Serial, Storage};


pub struct ArmFb {
    pub fb_info: FbInfo,
    cx: Cell<u32>,
    cy: Cell<u32>,
    color: Color,
}

impl ArmFb {
    pub fn new() -> Self {
        extern "C" {
            fn device_display_info() -> ((u32, u32), usize);
        }

        let info = unsafe { device_display_info() };

        return ArmFb {
            fb_info: FbInfo {
                name: "ARM Frame Buffer",
                fb_size: info.0,
                fb_addr: info.1 as *mut u32,
            },
            cx: Cell::new(0),
            cy: Cell::new(0),
            color: Color::Hex { d: 0xFFFFFF },
        };
    }

    pub fn clear_screen(&self) {
        for y in 0..self.fb_info.fb_size.1 {
            for x in 0..self.fb_info.fb_size.0 {
                unsafe {
                    self.graphics_pixel(self.color, x, y);
                }
            }
        }
    }
}

impl FrameBufferGraphics for ArmFb {
    fn graphics_pixel(&self, color: Color, x: u32, y: u32) {
        let mut cursor = self.fb_info.fb_addr as *mut u32;
        let color = match color {
            Color::Hex { d: color } => color,
            _ => 0,
        };

        let pos = x as isize + (y * self.fb_info.fb_size.0) as isize;

        unsafe {
            cursor = cursor.offset(pos);
            *cursor = color;
        }
    }
}

impl KernelConsoleDriver for ArmFb {}

impl KeyboardInput for ArmFb {}

impl Storage for ArmFb {}

impl Serial for ArmFb {}

impl Write for ArmFb {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        todo!()
    }
}

impl Led for ArmFb {}

impl Driver for ArmFb {
    fn driver_name(&self) -> &'static str {
        self.fb_info.name
    }

    fn name(&self) -> &'static str {
        FRAME_BUFFER
    }

    fn init(&self) -> DriverResult {
        Err("")
    }
}
