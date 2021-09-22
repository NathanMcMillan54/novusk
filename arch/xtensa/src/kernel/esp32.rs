use esp32_hal::hal::serial::Write;
use crate::kernel::device::{device, Device};
use esp32_hal::target::Peripherals;
use esp32_hal::serial::{Serial, Pins};
use esp32_hal::prelude::{GpioExt, FromValueType};
use esp32_hal::serial::config::Config;

pub struct Esp32;

impl Esp32 {
    pub fn new() -> Self {
        return Esp32;
    }

    fn serial_init(&mut self) {
        let peripherals = Peripherals::take().unwrap();
        let gpio = peripherals.GPIO.split();

        /* let serial = Serial::new(
            peripherals.UART0,
            Pins {
                    tx: gpio.gpio1,
                    rx: gpio.gpio3,
                    cts: None,
                    rts: None
                },
                Config {
                    baudrate: 115200.Hz(),
                    ..Config::default()
                }).unwrap(); */
    }

    pub fn enable_wdt(&mut self) {

    }
}

impl Device for Esp32 {
    fn get_peripherals(&mut self) {
        let peripherals = Peripherals::take();

        if peripherals.is_none() {
            panic!("Can't find peripherals");
        }
    }

    fn time_init(&mut self) {
        self.enable_wdt();
    }

    fn io_init(&mut self) {
        self.serial_init();
    }
}
