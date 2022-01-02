#![no_std]

#[macro_use] extern crate hifive1;

use device::{Device};
use core::any::Any;

pub(crate) mod hifive;
pub mod common;
pub(crate) mod lofive;

pub use hifive1::sprint;

pub struct HiFiveBoard;
pub struct LoFiveBoard;

impl HiFiveBoard {
    pub fn new() -> Self {
        let dev_res = hifive1::hal::DeviceResources::take();

        if dev_res.is_none() {
            panic!("Can't find device resources");
        }

        return HiFiveBoard;
    }

    fn init(&mut self) {
        self.serial_io_init();
        self.gpio_init();
    }
}

impl LoFiveBoard {
    pub fn new() -> Self {
        let dev_res = hifive1::hal::DeviceResources::take();

        if dev_res.is_none() {
            panic!("Can't find device resources");
        }

        return LoFiveBoard;
    }

    pub fn init(&mut self) {
        self.serial_io_init();
        self.gpio_init();
    }
}

