#![no_std]

#[macro_use] extern crate novuskinc;

#[path = "../../drivers/mailbox/mailbox.rs"]
mod mailbox;

use core::fmt::Write;
use mailbox::MailBoxSender;
use novuskinc::{fb::{FrameBuffer, FrameBufferGraphics}, mb::MailBox, serial::SerialIo};

#[no_mangle]
pub static mut KERNEL: Kernel = Kernel::empty();

pub struct Kernel<'a> {
    pub fb: FrameBuffer<'a>,
    pub mb: MailBox,
    pub serial: SerialIo,
}

impl <'a>Kernel<'a> {
    pub const fn empty() -> Self {
        return Kernel {
            fb: FrameBuffer::empty(),
            mb: MailBox::empty(),
            serial: SerialIo::empty(),
        };
    }

    pub fn set_serial(&mut self, kernel_serial: SerialIo) {
        self.serial = kernel_serial;
    }

    pub fn set_fb(&mut self, kernel_fb: FrameBuffer<'a>) {
        self.fb = kernel_fb;
    }

    pub fn set_mb(&mut self, kernel_mb: MailBox) {
        self.mb = kernel_mb;
    }

    pub fn get_serial_writer(&self) -> impl Write {
        return self.serial;
    }

    pub fn get_framebuffer(&self) -> FrameBuffer {
        return self.fb;
    }

    pub fn get_framebuffer_graphics(&'a self) -> &'static (dyn FrameBufferGraphics + 'a) {
        return self.fb.graphics;
    }

    pub fn get_mailbox_sender(&self) -> MailBox {
        return self.mb;
    }
}
