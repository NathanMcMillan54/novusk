use crate::board::RaspberryPi;

pub mod gpio;
pub mod led;

pub struct Rpi3 {
    gpio: gpio::RpiGpio,
}

impl Rpi3 {
    pub fn new() -> Self {
        return Rpi3 {
            gpio: gpio::RpiGpio::new()
        };
    }

    pub fn init(&self) {
        self.gpio_init();
    }
}
