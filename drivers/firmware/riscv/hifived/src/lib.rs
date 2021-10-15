#![no_std]

use device::Device;

pub struct Board;

impl Board {
    pub const UART0: usize = 0;

    pub fn new() -> Self {
        return Board;
    }

    pub fn device_init(&mut self) {

    }

    pub fn name(&mut self) -> &str {
        return "HiFive";
    }
}

impl Device for Board {
    fn name(&self) -> &'static str {
        return "HiFive";
    }
}
