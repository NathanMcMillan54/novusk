use crate::board::RaspberryPi;
use mailbox::MailBox;

pub mod gpio;
pub mod led;
pub mod mb;

pub struct Rpi3 {
    pub gpio: gpio::Rpi3Gpio,
    pub led: led::Rpi3Led,
    pub mb: mb::Rpi3Mb,
}

impl Rpi3 {
    pub fn new() -> Self {
        return Rpi3 {
            gpio: gpio::Rpi3Gpio::new(),
            led: led::Rpi3Led::new(),
            mb: mb::Rpi3Mb::new(),
        };
    }

    pub fn init(&self) {
        self.gpio_init();
        self.mb.init();
    }
}

impl RaspberryPi for Rpi3 {
    fn gpio_init(&self) {
        use core::ops::Deref;

        let gpio_deref = self.gpio.deref();

        // Check GPIO values
        if gpio_deref.__GPFSEL0 != 0 || gpio_deref.__GPFSEL1 != 73728 || gpio_deref.__GPFSEL3 != 0 || gpio_deref.__GPFSEL4 != 0 || gpio_deref.__GPFSEL5 != 0 {
            panic!("A GPIO value is wrong");
        }

        // Initialize anything that uses gpio pins
        self.led.init();
    }

    fn led_on(&self) {
        self.led.led_on();
    }

    fn led_off(&self) {
        self.led.led_off();
    }

    fn mailbox_init(&self) {
        self.mb.init();
    }
}