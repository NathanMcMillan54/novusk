use core::fmt::Write;
use novuskinc::drivers::{Driver, DriverResult};
use novuskinc::drivers::names::NONE;
use novuskinc::prelude::*;

/// An empty unimplemented driver
pub struct Empty;

impl KernelConsoleDriver for Empty {}

impl FrameBufferGraphics for Empty {}

impl KeyboardInput for Empty {}

impl Storage for Empty {}

impl Serial for Empty {}

impl Write for Empty {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        todo!()
    }
}

impl Led for Empty {}

impl Driver for Empty {
    fn driver_name(&self) -> &'static str {
        "Empty"
    }

    fn name(&self) -> &'static str {
        NONE
    }

    fn init(&self) -> DriverResult {
        Err("Unimplemented driver")
    }
}
