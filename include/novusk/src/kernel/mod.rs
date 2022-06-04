use crate::prelude::*;

pub mod macros;
pub mod types;

extern "C" {
    /// ``arch_prepare_init`` uses the architecture specific kernel to get ready for the main kernel
    /// initialization
    pub fn arch_prepare_init();

    /// This function is used to setup the architecture specific kernel
    pub fn setup_arch();

    /// Starts the main kernel, works for both the Novusk kernel and a device specific kernel
    pub fn start_kernel();

    /// ``kernel_init`` is the kernel's "main" function, it initializes all non architecture
    /// specific functions
    pub fn kernel_init();
}

pub struct Kernel<'a> {
    pub fb: FrameBuffer<'a>,
    pub firmware_interface: &'static dyn FirmwareInterface,
    pub serial: SimpleUart,
}

impl <'a>Kernel<'a> {
    pub fn set_serial(&mut self, kernel_serial: SimpleUart) {
        self.serial = kernel_serial;
    }

    pub fn set_fb(&mut self, kernel_fb: FrameBuffer<'a>) {
        self.fb = kernel_fb;
    }

    pub fn get_serial_writer(&self) -> impl core::fmt::Write {
        return self.serial;
    }

    pub fn get_framebuffer(&self) -> FrameBuffer {
        return self.fb;
    }

    pub fn get_framebuffer_graphics(&'a self) -> &'static (dyn FrameBufferGraphics + 'a) {
        return self.fb.graphics;
    }
}

