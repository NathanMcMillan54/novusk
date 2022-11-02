use core::fmt::Write;
use novuskinc::console::KernelConsoleDriver;
use novuskinc::drivers::{names::SERIAL, Driver, DriverResult};
use novuskinc::fb::FrameBufferGraphics;
use novuskinc::keyboard::KeyboardInput;
use novuskinc::led::Led;
use novuskinc::prelude::{Serial, Storage};
use crate::{HioDriver, io::{HIOWRITER, HioWriter}};

impl Write for HioDriver {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            HIOWRITER.write_str(s)
        }
    }
}

impl KernelConsoleDriver for HioDriver {}

impl FrameBufferGraphics for HioDriver {}

impl KeyboardInput for HioDriver {}

impl Led for HioDriver {}

impl Storage for HioDriver {}


impl Serial for HioDriver {
    fn read(&self) -> u8 {
        0
    }

    fn write(&self, byte: u8) {
        unsafe { HIOWRITER.write_char(byte as char); }
    }
}

impl Driver for HioDriver {
    fn driver_name(&self) -> &'static str {
        return "HIO (Semihosting I/O) Console Driver";
    }

    fn name(&self) -> &'static str {
        return SERIAL;
    }

    fn init(&self) -> DriverResult {
        unsafe {
            let hio = HioWriter::new();

            HIOWRITER.fd = hio.fd;

            if HIOWRITER.fd != 0 {
                return Ok(());
            } else { return Err(Default::default()); }
        }
    }
}