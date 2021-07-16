use cortex_m::asm::delay;
use defmt::debug;

pub struct NrfLed;

impl NrfLed {
    pub fn new() -> Self {
        return Self;
    }

    pub fn nrf52840_blink(&self) {
        use nrf52840_hal::gpio::{Level, p0::Parts};
        use nrf52840_hal::prelude::OutputPin;
        use nrf52840_pac::Peripherals;
        let peripherals = Peripherals::take().unwrap();
        let port0 = Parts::new(peripherals.P0);
        let mut led = port0.p0_17.into_push_pull_output(Level::Low);

        debug!("Led: On");
        led.set_high();
        delay(175000000);

        debug!("Led: Off");
        led.set_low();
    }

    pub fn blink(&self) {
        self.nrf52840_blink();
    }
}
