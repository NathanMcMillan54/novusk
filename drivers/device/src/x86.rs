use dif::Dif;
use novuskinc::prelude::*;
use crate::Device;

pub struct X86Device {
    pub dif: Dif,
    pub console: &'static mut dyn KernelConsoleDriver,
    pub display: &'static mut dyn FrameBufferGraphics,
    pub timer: &'static mut dyn Timer,
    pub keyboard: &'static mut dyn KeyboardInput,
    pub storage: &'static mut dyn Storage,
}

impl Device for X86Device {
    fn init(&mut self) {

    }
}
