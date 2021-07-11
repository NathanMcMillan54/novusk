use crate::kernel::device::Board;
use cortex_m::asm::delay;

pub struct NrfLed {
    board: Board,
}

impl NrfLed {
    pub fn new(&mut self) -> Self {
        return Self { board: Board::Nrf52840 }
    }

    pub fn nrf52840_blink(&self) {
        use nrf52840_hal::gpio::{Level, p0::Parts};
        use nrf52840_hal::prelude::OutputPin;
        use nrf52840_pac::Peripherals;
        let peripherals = Peripherals::take().unwrap();
        let port0 = Parts::new(peripherals.P0);
        let mut led = port0.p0_17.into_push_pull_output(Level::Low);

        led.set_high();
        delay(750);
        led.set_low();
    }

    /* pub fn nrf52832_blink(&self) {
        use nrf52832_hal::gpio::{Level, p0::Parts};
        use nrf52832_hal::prelude::OutputPin;
        use nrf52832_pac::Peripherals;

        let peripherals = Peripherals::take().unwrap();
        let port0 = Parts::new(peripherals.P0);
        let mut led = port0.p0_17.into_push_pull_output(Level::Low);

        led.set_high().unwrap();
        delay(750);
        led.set_low();
    } */
}
