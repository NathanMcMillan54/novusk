use core::fmt::{Arguments, Write};
use nrf52840_hal::uarte::{Baudrate, Pins, Parity, Uarte};
use nrf52840_hal::pac::{Peripherals, UARTE0};
use nrf52840_hal::gpio::{Level, p0::Parts};

pub struct NrfUart {
    pub uarte: Uarte<UARTE0>,
}

impl NrfUart {
    pub fn init() -> Self {
        let peripherals = Peripherals::take().unwrap();

        let (uart0, cdc_pins) = {
            let parts = Parts::new(peripherals.P0);
            (
                peripherals.UARTE0,
                Pins {
                    txd: parts.p0_06.into_push_pull_output(Level::High).degrade(),
                    rxd: parts.p0_08.into_floating_input().degrade(),
                    cts: Some(parts.p0_07.into_floating_input().degrade()),
                    rts: Some(parts.p0_05.into_push_pull_output(Level::High).degrade()),
                },
            )
        };

        return NrfUart { uarte: Uarte::new(uart0, cdc_pins, Parity::EXCLUDED, Baudrate::BAUD115200) };
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.uarte.write(bytes);
    }
}
