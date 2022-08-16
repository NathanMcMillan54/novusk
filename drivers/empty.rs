use novuskinc::drivers::{Driver, DriverResult};
use novuskinc::drivers::names::NONE;
use novuskinc::prelude::*;

/// An empty unimplemented driver
pub struct Empty;

impl KernelConsoleDriver for Empty {}

impl FrameBufferGraphics for Empty {}

impl KeyboardInput for Empty {}

impl Storage for Empty {}

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