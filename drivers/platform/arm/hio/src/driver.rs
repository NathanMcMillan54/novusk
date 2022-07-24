use core::fmt::Write;
use novuskinc::console::KernelConsoleDriver;
use novuskinc::drivers::{names::CONSOLE, Driver, DriverResult};
use novuskinc::fb::FrameBufferGraphics;
use novuskinc::keyboard::KeyboardInput;
use novuskinc::led::Led;
use novuskinc::prelude::Storage;
use crate::{HioDriver, io::{HIOWRITER, HioWriter}};

impl Write for HioDriver {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            HIOWRITER.write_str(s)
        }
    }
}

impl KernelConsoleDriver for HioDriver {
    fn write_character(&self, c: char, x: u16, y: u16) {
        unsafe {
            HIOWRITER.write_char(c);
        }
    }

    fn write_string(&self, string: &str, x: u16, y: u16) {
        unsafe {
            for b in string.as_bytes() {
                self.write_character(*b as char, 0, 0);
            }
        }
    }

    fn new_line(&self) {
        unsafe {
            self.write_string("\n", 0, 0);
        }
    }

    fn clear_screen(&self, option: u16) {
        for _ in 0..10 {
            self.new_line();
        }
    }
}

impl FrameBufferGraphics for HioDriver {}

impl KeyboardInput for HioDriver {}

impl Led for HioDriver {}

impl Storage for HioDriver {}

impl Driver for HioDriver {
    fn driver_name(&self) -> &'static str {
        return "HIO (Semihosting I/O) Console Driver";
    }

    fn name(&self) -> &'static str {
        return CONSOLE;
    }

    fn init(&self) -> Option<DriverResult> {
        unsafe {
            let hio = HioWriter::new();

            HIOWRITER.fd = hio.fd;

            if HIOWRITER.fd != 0 {
                return Some(Ok(()));
            } else { return Some(Err(Default::default())); }
        }
    }
}
