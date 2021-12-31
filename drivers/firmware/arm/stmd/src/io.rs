use core::fmt::Write;
use core::ops::Deref;
use kinfo::status::set_status;
use crate::BOARD_MODLE;
use crate::device::{pac::{Peripherals, USART2}, prelude::*, serial::{config::Config, Serial, Tx}};

pub fn get_serial() -> Tx<USART2> {
    let peripherals = Peripherals::take().unwrap();
    let gpioa = peripherals.GPIOA.split();
    let rcc = peripherals.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.mhz()).freeze();
    let tx_pin = gpioa.pa2.into_alternate();

    let mut tx_uart = Serial::tx(
        peripherals.USART2,
        tx_pin,
        Config::default().baudrate(9600.bps()),
        clocks,
    );

    if !tx_uart.is_ok() {
        unsafe { set_status("not ok"); }
        printk!("Couldn't get serial for STM32f4{}\n", BOARD_MODLE);
    }

    return tx_uart.unwrap();
}

pub(crate) fn io_init() {
    get_serial();
}
