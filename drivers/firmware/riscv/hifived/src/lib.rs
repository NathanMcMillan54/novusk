#![no_std]

#[macro_use] extern crate hifive1;
#[macro_use] extern crate novuskinc;

use hifive1::{clock, stdout};
use hifive1::hal::{DeviceResources, time::U32Ext};

use device::Device;

pub struct Board;

impl Board {
    pub const UART0: usize = 0;

    pub fn new() -> Self {
        let device_resources = DeviceResources::take();

        if device_resources.is_none() {
            panic!("Can't find device resources");
        }

        return Board;
    }

    pub fn device_init(&mut self) {
        self.serial_io_init();
        self.gpio_init();
    }

    pub fn name(&mut self) -> &str {
        return "HiFive";
    }
}

impl Device for Board {
    fn serial_io_init(&self) {
        let device_resources = DeviceResources::take().unwrap();
        let peripherals = device_resources.peripherals;
        let pins = device_resources.pins;

        let clocks = clock::configure(peripherals.PRCI, peripherals.AONCLK, 320.mhz().into());

        stdout::configure(
            peripherals.UART0,
            pin!(pins, uart0_tx),
            pin!(pins, uart0_rx),
            115_200.bps(),
            clocks,
        );
    }

    fn gpio_init(&self) {

    }
}
