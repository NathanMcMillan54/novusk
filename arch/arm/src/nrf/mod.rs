pub mod onboard;
pub use onboard as nrf_board;
use onboard::NrfLed;
use usb::{set_usb_driver, DriverNames};

pub fn nrf_init() {
    unsafe { set_usb_driver(DriverNames::Nrf); }

    let mut led = NrfLed::new();
    led.nrf52840_blink();
}
