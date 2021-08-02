use core::fmt::Arguments;
use hifive1::{pin, pins, sprintln, clock, stdout};
use hifive1::hal::DeviceResources;
use hifive1::hal::clock::Clocks;
use hifive1::hal::prelude::*;
use hifive1::hal::time::U32Ext;
use super::uart::Uart;

pub struct Board;

impl Board {
    pub fn board_init(&mut self) {
        self.uart_init();
    }

    pub fn clocks(&mut self) -> Clocks {
        let dr = DeviceResources::take().unwrap();
        let p = dr.peripherals;

        let clocks = clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());
        return clocks;
    }
}

impl Uart for Board {
    fn uart_init(&mut self) {
        let dr = DeviceResources::take().unwrap();
        let p = dr.peripherals;
        let pins = dr.pins;

        let clock = self.clocks();

        stdout::configure(
            p.UART0,
            pin!(pins, uart0_tx),
            pin!(pins, uart0_rx),
            115_200.bps(),
            clock,
        );
    }
}

pub fn write_fmt(fmt: Arguments) {
    sprintln!("{}", fmt);
}
