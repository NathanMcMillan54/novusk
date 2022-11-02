use dif::Dif;
use novuskinc::drivers::Driver;
use novuskinc::prelude::*;
use crate::Device;

/// An ARM 32/64 device struct
pub struct ArmDevice {
    pub dif: Dif,
    pub mailbox: Option<&'static dyn Driver>,
    pub console: Option<&'static dyn Driver>,
    pub serial: Option<&'static dyn Driver>,
    pub display: Option<&'static dyn Driver>,
    pub timer: Option<&'static dyn Driver>,
    pub keyboard: Option<&'static dyn Driver>,
}

impl Device for ArmDevice {
    fn init(&mut self) {
        // *self.mailbox.init();
    }
}
