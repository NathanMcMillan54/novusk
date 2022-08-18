use dif::Dif;
use novuskinc::prelude::*;
use crate::Device;

pub struct X86Device {
    pub dif: Dif,
    pub console: Option<&'static dyn KernelConsoleDriver>,
    pub display: Option<&'static dyn FrameBufferGraphics>,
    pub timer: Option<&'static dyn Timer>,
    pub keyboard: Option<&'static dyn KeyboardInput>,
    pub storage: Option<&'static dyn Storage>,
}

impl Device for X86Device {
    fn init(&mut self) {

    }
}
