use bcm::bcm2837::serial::BCM2837_SIMPLEUART;
use core::fmt::Write;
use novuskinc::console::KernelConsoleDriver;
use novuskinc::drivers::{Driver, DriverResult};
use novuskinc::fb::FrameBufferGraphics;
use novuskinc::keyboard::KeyboardInput;
use novuskinc::serial::early_serial_init;

pub struct Rpi3Console;

impl Write for Rpi3Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe { BCM2837_SIMPLEUART.write_str(s) }
    }
}

impl KernelConsoleDriver for Rpi3Console {
    fn write_character(&self, c: char, x: u16, y: u16) {
        unsafe { BCM2837_SIMPLEUART.serial_write_byte(c as u8); }
    }

    fn write_string(&self, string: &str, x: u16, y: u16) {
        for c in string.as_bytes() {
            self.write_character(*c as char, 0, 0);
        }
    }
}

impl FrameBufferGraphics for Rpi3Console {}

impl KeyboardInput for Rpi3Console {}

impl Driver for Rpi3Console {
    fn driver_name(&self) -> &'static str {
        return "RPi3 Console Driver";
    }

    fn name(&self) -> &'static str {
        return "Console Driver";
    }

    fn init(&self) -> Option<DriverResult> {
        unsafe {
            if early_serial_init() == 0 {
                return Some(Ok(()));
            } else if early_serial_init() == 1 {
                return Some(Err(Default::default()));
            } else { return None; }
        }
    }
}
