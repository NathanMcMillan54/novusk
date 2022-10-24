use bcm_soc::gpio::*;
use tock_registers::interfaces::Writeable;

pub struct Rpi3Led {
    pub gpio: Bcm2837Gpio,
}

impl Rpi3Led {
    pub fn new() -> Self {
        return Rpi3Led { gpio: Bcm2837Gpio::new() };
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
        self.led_on();

        for _ in 0..sleep_time { }

        self.led_off();
    }
}
