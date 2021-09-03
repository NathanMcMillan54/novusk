use libbmu::Time;
use crate::gpio::*;
use tock_registers::interfaces::Writeable;

pub struct RpiLed {
    pub gpio: RpiGpio,
}

impl RpiLed {
    pub fn new() -> Self {
        return RpiLed { gpio: RpiGpio::new() };
    }

    pub fn init(&mut self) {
        self.gpio.GPFSEL2.write(GPFSEL2::FSEL29::Output);
    }

    pub fn blink(&mut self, sleep_time: usize) {
        let mut time = Time::new();

        self.gpio.GPSET0.write(GPSET0::O29::SET);
        time.sleepc(sleep_time as i64);

        self.gpio.GPCLR0.write(GPCLR0::O29::CLEAR);
        time.sleepc(sleep_time as i64);
    }
}

fn blink(sleep: usize) {
    let mut led = RpiLed::new();
    led.init();
    led.blink(sleep);
}

define_led_blink_function!(blink);
