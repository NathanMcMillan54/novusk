use super::gpio::*;

pub struct Led {
    gpio: Gpio,
}

impl Led {
    pub fn new() -> Self {
        return Self { gpio: Gpio };
    }

    pub fn init(&self) {
        self.gpio.GPFSEL2.modify(GPFSEL2::FSEL29::Output);
    }

    pub fn on(&self) {
        self.gpio.GPSET0.write(GPSET0::O29::Set);
    }

    pub fn off(&self) {
        self.gpio.GPCLR0.write(GPCLR0::O29::Clear);
    }
}
