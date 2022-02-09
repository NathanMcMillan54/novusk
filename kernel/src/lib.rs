#![no_std]

#[macro_use] extern crate novuskinc;

#[path = "../../drivers/mailbox/mailbox.rs"]
mod mailbox;

use core::fmt::Write;
use mailbox::MailBoxSender;
use novuskinc::{fb::FrameBuffer, mb::MailBox, serial::SerialIo};

#[no_mangle]
pub static mut KERNEL: Kernel = Kernel::empty();

#[derive(Debug, Copy, Clone)]
pub struct Kernel {
    pub fb: FrameBuffer,
    pub mb: MailBox,
    pub serial: SerialIo,
}

impl Kernel {
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

    pub fn set_fb(&mut self, kernel_fb: FrameBuffer) {
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

    pub fn get_mailbox_sender(&self) -> impl MailBoxSender {
        return self.mb;
    }
}
