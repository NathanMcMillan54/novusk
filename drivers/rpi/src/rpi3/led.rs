use libbmu::Time;
use crate::{gpio::*, RaspberryPi, Rpi3};
use tock_registers::interfaces::Writeable;

pub struct Rpi3Led {
    pub gpio: Rpi3Gpio,
}

impl Rpi3Led {
    pub fn new() -> Self {
        return Rpi3Led { gpio: Rpi3Gpio::new() };
    }

    pub fn init(&self) {
        self.gpio.GPFSEL2.write(GPFSEL2::FSEL29::Output);
    }

    pub fn led_on(&self) {
        self.gpio.GPSET0.write(GPSET0::O29::SET);
    }

    pub fn led_off(&self) {
        self.gpio.GPCLR0.write(GPCLR0::O29::CLEAR);
    }

    pub fn blink(&mut self, sleep_time: usize) {
        let mut time = Time::new();

        self.led_on();
        time.sleepc(sleep_time as i64);
        self.led_off();
    }
}
