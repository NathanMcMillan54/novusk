#![no_std]

#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;

use kinfo::status::set_status;
use nrf52840_pac::Peripherals;

pub mod uart;
pub use uart::NrfUartIo;

pub fn nrf_board_init() {
    if Peripherals::take().is_none() {
        panic!("Can't find Nrf peripherals, maybe you're using the wrong board");
    } else {  }
}

