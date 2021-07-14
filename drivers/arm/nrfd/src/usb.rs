use nrf52840_pac::Peripherals;
use nrf_usbd::{Usbd, UsbPeripheral};

pub struct Usb;

impl Usb {
    pub fn new() -> Self {
        return Self;
    }

    pub fn read(&mut self) -> u8 {
        // let usbd = Usbd::new();

        return 0;
    }
}
