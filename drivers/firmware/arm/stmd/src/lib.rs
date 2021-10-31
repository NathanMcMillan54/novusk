#![no_std]

#[macro_use] extern crate novuskinc;

use stm32f4xx_hal::pac::Peripherals;

#[cfg(feature = "stm32f401")]
pub(crate) const BOARD_MODLE: i32 = 01;
#[cfg(feature = "stm32f407")]
pub(crate) const BOARD_MODLE: i32 = 07;

pub mod io;
pub mod net;

pub mod board {
    pub use crate::net::{is_supported, net_init};
}

#[no_mangle]
pub extern "C" fn device_init() -> (Result<(), &'static str>, &'static str) {
    let mut peripherals = Peripherals::take();

    unsafe { Peripherals::steal(); }

    if peripherals.is_none() {
        panic!("Can't find peripherals");
    } else { printk!("Got peripherals"); }

    #[cfg(not(feature = "qemu"))]
    io::io_init();

    return (Ok(()), "STM32");
}
