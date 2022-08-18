#![no_std]
#![feature(trait_upcasting)]

#[macro_use] extern crate hifive1;
#[macro_use] extern crate novuskinc;

use device::{riscv::RiscVDevice, Device};
use core::any::Any;

pub(crate) mod hifive;
pub mod common;
pub(crate) mod lofive;

extern "C" {
    pub(crate) static mut RISCV_DEVICE: RiscVDevice;
}

pub struct HiFiveBoard;
pub struct LoFiveBoard;

impl HiFiveBoard {
    pub fn new() -> Self {
        let dev_res = hifive1::hal::DeviceResources::take();

        return HiFiveBoard;
    }

    fn init(&mut self) {
        //self.serial_io_init();
        //self.gpio_init();
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
        //self.serial_io_init();
        //self.gpio_init();
    }
}

#[no_mangle]
pub extern "C" fn device_init() -> (Result<(), &'static str>, &'static str) {
    #[cfg(feature = "hifive")]
    let mut dev = HiFiveBoard::new();

    #[cfg(feature = "lofive")]
    let mut dev = LoFiveBoard::new();

    return (Ok(()), "");
}
