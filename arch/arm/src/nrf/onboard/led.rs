use crate::kernel::device::Board;
use crate::kernel::kernel::{arm32_printk, dprint};
use cortex_m::asm::delay;

pub struct NrfLed {
    pub(crate) board: Board,
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

        dprint!("Led: On");
        led.set_high();
        delay(175000000);

        dprint!("Led: Off");
        led.set_low();
    }

    pub fn blink(&self) {
        match self.board {
            Board::Nrf52840 =>
                self.nrf52840_blink(),
            _ =>
                arm32_printk!("{:?} doesn't support NrfLed::blink", self.board)
        }
    }
}
