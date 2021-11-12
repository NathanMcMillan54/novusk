use device::Device;

pub mod led;
use led::RpiLed;

pub struct Rpi3 {
    pub led: RpiLed,
}

impl Rpi3 {
    pub fn new() -> Self {
        return Rpi3 { led: RpiLed::new(), };
    }

    pub fn init(&mut self) {

    }

    pub fn led_on(&mut self) {
        self.led.led_on();
    }

    pub fn led_off(&mut self) {
        self.led.led_off();
    }
}

impl Device for Rpi3 {
    fn name(&self) -> &'static str {
        return "Rpi 3";
    }
}
