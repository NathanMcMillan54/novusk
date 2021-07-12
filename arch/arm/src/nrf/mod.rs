pub mod onboard;
use onboard::led::NrfLed;
use crate::kernel::device::DEVICE;

pub fn nrf_init() {
    let mut led = unsafe { NrfLed::new(&mut NrfLed { board: DEVICE }) };

    led.nrf52840_blink();
}
