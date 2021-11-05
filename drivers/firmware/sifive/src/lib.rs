#![no_std]

use device::{Device};

pub(crate) mod hifive;
pub(crate) mod lofive;

pub struct HiFiveBoard;
pub struct LoFiveBoard;

pub fn board_init() {
    #[cfg(feature = "hifive")]
    HiFiveBoard::new().init();

    #[cfg(feature = "lofive")]
    LoFiveBoard::new().init();
}

impl HiFiveBoard {
    pub fn new() -> Self {
        return HiFiveBoard;
    }

    fn init(&mut self) {
        self.serial_io_init();
        self.gpio_init();
    }
}

impl LoFiveBoard {
    pub fn new() -> Self {
        return LoFiveBoard;
    }

    pub fn init(&mut self) {
        self.serial_io_init();
        self.gpio_init();
    }
}