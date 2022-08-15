use dif::Dif;
use novuskinc::prelude::*;
use crate::Device;

/// An ARM 32/64 device struct
pub struct ArmDevice {
    pub dif: Dif,
    pub mailbox: &'static dyn FirmwareInterface,
    pub console: &'static dyn KernelConsoleDriver,
    pub display: Option<&'static dyn FrameBufferGraphics>,
    pub timer: Option<&'static dyn Timer>,
    pub keyboard: Option<&'static dyn KeyboardInput>,
    pub storage: Option<&'static dyn Storage>,
}

impl Device for ArmDevice {
    fn init(&mut self) {
        self.mailbox.init();
    }
}
