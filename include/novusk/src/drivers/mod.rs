use core::fmt::{Arguments, Write};
use crate::prelude::*;

pub mod manager;
pub mod names;

pub type DriverResult = Result<(), &'static str>;

pub trait Driver: KernelConsoleDriver + FrameBufferGraphics + KeyboardInput + Storage {
    /// The ``driver_name`` function should be used to return the driver's name
    ///
    /// Example:
    /// ```rust
    ///     fn driver_name(&self) -> &'static str {
    ///         return "ANCD (A New Console Driver)";
    ///     }
    ///
    fn driver_name(&self) -> &'static str { return "None" }

    /// The ``name`` should return the "type" of driver it is so the kernel can easily index it
    /// in the driver manager
    ///
    /// Example:
    /// ```rust
    ///     fn name(&self) -> &'static str {
    ///         return "Console Driver" // This is for a console driver
    ///     }
    /// ```
    ///
    /// [This page](link) has a list of driver names and what they're used for.
    fn name(&self) -> &'static str { return "None"; }

    /// The init function needs to be used to initialize the driver, it is what the kernel calls to
    /// start it.
    fn init(&self) -> DriverResult {
        return Err("Unimplemented");
    }
}
