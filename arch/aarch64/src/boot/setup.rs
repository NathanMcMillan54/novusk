use core::borrow::Borrow;
use novuskinc::platform::{early_device_init, DEVICE_INIT_ERRORS};
use novuskinc::serial::early_serial_init;
pub(crate) struct Aarch64Boot;

impl Aarch64Boot {
    pub fn new() -> Self {
        return Aarch64Boot;
    }

    pub fn setup(&self) {
        //unsafe { set_dif(); }
    }
}
