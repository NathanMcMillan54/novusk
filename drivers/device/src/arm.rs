use dif::Dif;
use novuskinc::drivers::Driver;
use novuskinc::prelude::*;
use crate::Device;

/// An ARM 32/64 device struct
pub struct ArmDevice {
    pub dif: Dif,
    pub mailbox: Option<&'static mut dyn Driver>,
    pub console: Option<&'static mut dyn Driver>,
    pub serial: Option<&'static mut dyn Driver>,
    pub simple_uart: Option<&'static mut dyn Driver>,
    pub display: Option<&'static mut dyn Driver>,
    pub timer: Option<&'static mut dyn Driver>,
    pub keyboard: Option<&'static mut dyn Driver>,
}

impl Device for ArmDevice {
    fn init(&mut self) {
        // *self.mailbox.init();
    }
}
