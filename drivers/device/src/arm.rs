use dif::Dif;
use novuskinc::prelude::*;
use crate::Device;

/// An ARM 32/64 device struct
pub struct ArmDevice {
    pub dif: Dif,
    pub mailbox: &'static mut dyn FirmwareInterface,
    pub console: &'static mut dyn KernelConsoleDriver,
    pub display: Option<&'static mut dyn FrameBufferGraphics>,
    pub timer: Option<&'static mut dyn Timer>,
    pub keyboard: Option<&'static mut dyn KeyboardInput>,
}

impl Device for ArmDevice {
    fn init(&mut self) {
        // *self.mailbox.init();
    }
}
