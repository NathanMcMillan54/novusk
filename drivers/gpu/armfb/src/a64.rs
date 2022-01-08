use core::sync::atomic::{compiler_fence, Ordering};
use fb::{Fb, FbColor, FrameBufferGraphics};
use rpi::common::*;

pub struct A64Fb {
    pub fb: Fb,
    pub width: u32,
    pub height: u32,
    pub ptr: usize,
}

impl A64Fb {
    pub fn new() -> Self {
        return A64Fb {
            fb: Fb::new("A64Fb", 0x0),
            width: 0,
            height: 0,
            ptr: 0x0,
        }
    }

    pub fn init(&mut self) {
        let mut mb = RpiMb::new();

        mb.mb_buffer[0] = 35 * 4;
        mb.mb_buffer[1] = REQUEST;

        mb.mb_buffer[2] = tag::SETPHYWH;
        mb.mb_buffer[3] = 8;
        mb.mb_buffer[4] = 8;
        mb.mb_buffer[5] = 1024;
        mb.mb_buffer[6] = 768;

        mb.mb_buffer[7] = tag::SETVIRTWH;
        mb.mb_buffer[8] = 8;
        mb.mb_buffer[9] = 8;
        mb.mb_buffer[10] = 1024;
        mb.mb_buffer[11] = 768;

        mb.mb_buffer[12] = tag::SETVIRTOFFSET;
        mb.mb_buffer[13] = 8;
        mb.mb_buffer[14] = 8;
        mb.mb_buffer[15] = 0;
        mb.mb_buffer[16] = 0;

        mb.mb_buffer[17] = tag::SETDEPTH;
        mb.mb_buffer[18] = 4;
        mb.mb_buffer[19] = 4;
        mb.mb_buffer[20] = 32;

        mb.mb_buffer[21] = tag::SETPXORDER;
        mb.mb_buffer[22] = 4;
        mb.mb_buffer[23] = 4;
        mb.mb_buffer[24] = 0;

        mb.mb_buffer[25] = tag::GETFB;
        mb.mb_buffer[26] = 8;
        mb.mb_buffer[27] = 8;
        mb.mb_buffer[28] = 4096;
        mb.mb_buffer[29] = 0;

        mb.mb_buffer[30] = tag::GETPITCH;
        mb.mb_buffer[31] = 4;
        mb.mb_buffer[32] = 4;
        mb.mb_buffer[33] = 0;

        mb.mb_buffer[34] = tag::LAST;

        compiler_fence(Ordering::Release);

        mb.call(channel::PROP);

        self.width = mb.mb_buffer[5];
        self.height = mb.mb_buffer[6];
        self.ptr = mb.mb_buffer[28] as usize;

        self.fb = Fb::new("RPi3 Fb", self.ptr);
    }
}

impl FrameBufferGraphics for A64Fb {
    fn pixel(&self, x: usize, y: usize, color: FbColor) {
        let mut cursor = self.ptr as *mut u32;

        let pos: isize = x as isize + (y as isize * self.width as isize);

        unsafe {
            cursor = cursor.offset(pos);
            *cursor = color.r as u32;
        }
    }

    fn clear_screen(&self, color: FbColor) {
        let mut cursor = self.ptr as *mut u32;

        for y in 0..self.height {
            for x in 0..self.width {
                unsafe {
                    *cursor = color.r as u32;

                    cursor = cursor.offset(1);
                }
            }
        }
    }
}

pub fn a64_fb_init() {
    let mut fb = A64Fb::new();

    fb.init();
    fb.clear_screen(FbColor::new(0x000000, 0, 0));
}
