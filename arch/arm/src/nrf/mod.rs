pub mod onboard;
pub use onboard as nrf_board;

use alloc::string::ToString;
use crate::kernel::kernel::arm32_printk;
use onboard::NrfLed;

pub fn nrf_init() {
    let mut led = NrfLed::new();
    led.nrf52840_blink();
    arm32_printk!("    LED blink tested");
}
