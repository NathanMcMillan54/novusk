use core::ops::Deref;
use uefi::prelude::BootServices;
use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput, Mode};

pub struct GopWriter {
    pub width: usize,
    pub height: usize,
}

impl GopWriter {
    pub fn new() -> Self {
        return GopWriter {
            width: 1024,
            height: 768,
        };
    }

    pub fn init(&self, bt: &BootServices) {
        let gop = self.get_gop(bt);
        let mode = gop.modes().map(|mode| mode.expect("Failed to get GOP mode")).find(|mode| {
            let mode_info = mode.info();
            mode_info.resolution() == (self.width, self.height)
        }).unwrap();

        gop.set_mode(&mode);
    }

    pub fn get_gop(&self, bt: &BootServices) -> &mut GraphicsOutput {
        let gop = unsafe { &mut *bt.locate_protocol::<GraphicsOutput>().unwrap().unwrap().get() };

        return gop;
    }

    pub fn clear_screen(&self, bt: &BootServices, color: (u8, u8, u8)) {
        let gop = self.get_gop(bt);

        gop.blt(BltOp::VideoFill {
            color: BltPixel::new(color.0, color.1, color.2),
            dest: (0, 0),
            dims: (self.width, self.height),
        });
    }

    pub fn pixel(&self, bt: &BootServices, color: (u8, u8, u8)) {
        let gop = self.get_gop(bt);

        let mut fb = gop.frame_buffer();

        unsafe { fb.write_value(0, [1, 1, 1]); }
    }
}
