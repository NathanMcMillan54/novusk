pub mod onboard;
use onboard::led::NrfLed;
use crate::kernel::device::DEVICE;
use nrf52840_pac::Peripherals;
use usb::{set_usb_driver, DriverNames};

pub fn nrf_init() {
    unsafe { set_usb_driver(DriverNames::Nrf); }

    let mut led = unsafe { NrfLed::new(&mut NrfLed { board: DEVICE }) };

    led.nrf52840_blink();
}
