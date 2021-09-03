use libbmu::Time;
use nrf52840_hal::gpio::{Level, p0::Parts};
use nrf52840_hal::pac::Peripherals;
use nrf52840_hal::prelude::OutputPin;

pub struct NrfLed;

impl NrfLed {
    pub fn new() -> Self {
        return NrfLed;
    }

    pub fn blink(&mut self, sleep: usize) {
        let mut time = Time::new();

        let mut peripherals = Peripherals::take().unwrap();
        let mut parts = Parts::new(peripherals.P0);

        let mut led = parts.p0_17.into_push_pull_output(Level::Low);

        led.set_high();
        time.sleepc(sleep as i64);
        led.set_low();
    }
}

fn blink(sleep: usize) {
    let mut nrf_led = NrfLed::new();
    nrf_led.blink(sleep);
}

define_led_blink_function!(blink);
