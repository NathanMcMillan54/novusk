use core::fmt::Write;
use dif::DifFieldNames;
use novuskinc::serial::early_serial_init;

pub struct ArmBootSetup;

impl ArmBootSetup {
    pub fn new() -> Self {
        return ArmBootSetup;
    }

    pub unsafe fn setup(&self) {

    }
}