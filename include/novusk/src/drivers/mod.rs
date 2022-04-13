use core::fmt::Write;
use crate::prelude::*;

pub mod manager;

pub type DriverResult = Result<(), &'static str>;

pub trait Driver: Write + KernelConsoleDriver + FrameBufferGraphics + KeyboardInput {
    /// The driver_name should return the "type" of driver it is so the kernel can easily index it
    /// in the driver manager
    ///
    /// Example:
    /// ```rust
    ///     fn driver_name(&self) -> &'static str {
    ///         return "Console Driver" // This is for a console driver
    ///     }
    fn driver_name(&self) -> &'static str {
        return "None"
    }

    /// The name function should be used to return the driver's name
    ///
    /// Example:
    /// ```rust
    ///     fn name(&self) -> &'static str {
    ///         return "ANCD (A New Console Driver)";
    ///     }
    fn name(&self) -> &'static str { return "None"; }

    /// The init function needs to be used to initialize the driver, it is what the kernel calls to
    /// start it.
    fn init(&self) -> Option<DriverResult> {
        return None;
    }
}