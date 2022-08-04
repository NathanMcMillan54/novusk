#![no_std]

#[macro_use] extern crate novuskinc;

#[cfg(feature = "stm32f407")]
#[macro_use] extern crate stm32f4xx_hal;

#[cfg(feature = "stm32f407")]
use stm32f4xx_hal::stm32::Peripherals;

#[cfg(feature = "stm32f407")]
pub(crate) const BOARD_MODLE: &'static str = "STM32f407";

pub(crate) mod device;
pub mod io;
pub mod irq;
pub mod net;

pub mod board {
    pub use crate::net::{is_supported, net_init};
}

#[no_mangle]
pub extern "C" fn device_init() -> (Result<(), &'static str>, &'static str) {
    let mut peripherals = Peripherals::take();

    if peripherals.is_none() {
        panic!("Can't find peripherals");
    } else { printk!("Got peripherals"); }

    #[cfg(not(feature = "qemu"))]
    io::io_init();

    return (Ok(()), BOARD_MODLE);
}
