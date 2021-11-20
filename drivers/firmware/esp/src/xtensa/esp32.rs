use core::fmt;
use device::Device;
use esp32_hal::clock_control::{ClockControl, XTAL_FREQUENCY_AUTO};
use esp32_hal::dport::Split;
use esp32_hal::gpio::{GpioExt, Unknown, Gpio3, Gpio1};
use esp32_hal::hal::{serial::Write, watchdog::WatchdogDisable};
use esp32_hal::prelude::FromValueType;
use esp32_hal::target::{Peripherals, UART1};
use esp32_hal::timer::Timer;
use esp32_hal::serial::{Error, Pins, Serial, Tx, config::Config};

pub struct Esp32;

impl Esp32 {
    pub fn new() -> Self {
        return Esp32;
    }

    pub fn init(&mut self) -> Result<(), &str> {
        let peripherals = Peripherals::take();

        if peripherals.is_none() {
            return Err("Cannot find device peripherals");
        }

        self.serial_io_init();

        return Ok(());
    }

    pub fn get_serial(&self) -> Serial<UART1, Gpio1<Unknown>, Gpio3<Unknown>> {
        let peripherals = Peripherals::take().unwrap();
        let pins = peripherals.GPIO.split();

        let serial: Serial<_, _, _> = Serial::new(
            peripherals.UART1,
            Pins {
                tx: pins.gpio1,
                rx: pins.gpio3,
                cts: None,
                rts: None,
            },
            Config {
                baudrate: 115200.Hz(),
                ..Config::default()
            },
            self.get_clock_control().freeze().unwrap().0,
        ).unwrap();

        return serial;
    }

    pub fn get_clock_control(&self) -> ClockControl {
        let peripherals = Peripherals::take().unwrap();

        let clock_control = ClockControl::new(peripherals.RTCCNTL, peripherals.APB_CTRL,peripherals.DPORT.split().1,XTAL_FREQUENCY_AUTO).unwrap();

        return clock_control;
    }
}

impl Device for Esp32 {
    fn name(&self) -> &'static str {
        return "ESP32";
    }

    fn serial_io_init(&self) {
        let (mut tx, mut rx) = self.get_serial().split();

        self.write_bytes(b"Serial I/O initialized\n");
    }

    fn time_init(&self) {

    }

    fn disable_wdt(&self) {
        let peripherals = Peripherals::take().unwrap();

        let (clock_config, mut watchdog_timer) = self.get_clock_control().freeze().unwrap();
        let (_, _, _, mut watchdog0) = Timer::new(peripherals.TIMG0, clock_config);
        let (_, _, _, mut watchdog1) = Timer::new(peripherals.TIMG1, clock_config);

        watchdog_timer.disable();
        watchdog0.disable();
        watchdog1.disable();
    }

    fn enable_wdt(&self) {

    }

    fn write_bytes(&self, bytes: &[u8]) {
        let (mut tx, mut rx) = self.get_serial().split();

        for byte in bytes {
            tx.write(*byte);
        }
    }
}
