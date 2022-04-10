use core::fmt::Write;
use crate::prelude::*;

pub mod manager;

pub type DriverResult = Result<(), &'static str>;

pub trait Driver: KernelConsoleDriver + FrameBufferGraphics + KeyboardInput {
    fn name(&self) -> &'static str {
        return "None"
    }

    fn init(&self) -> Option<DriverResult> {
        return None;
    }
}