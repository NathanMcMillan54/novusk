use esp32_hal::gpio::{Gpio2, GpioExt, Output, PushPull};
use esp32_hal::target::Peripherals;

pub struct Led;

impl Led {
    pub fn new() -> Gpio2<Output<PushPull>> {
        let p = Peripherals::take().expect("Couldn't take peripherals");
        let pins = p.GPIO.split();
        let mut led = pins.gpio2.into_push_pull_output();
        return led;
    }
}
