pub mod onboard;
pub use onboard as nrf_board;

use alloc::string::ToString;
use crate::kernel::kernel::arm32_printk;
use onboard::NrfLed;
use usb::{set_usb_driver, DriverNames};

pub fn nrf_init() {
    let mut led = NrfLed::new();
    led.nrf52840_blink();
    arm32_printk!("    LED blink tested");

    unsafe { set_usb_driver(DriverNames::Nrf); }
    arm32_printk!("    USB driver set");
}
