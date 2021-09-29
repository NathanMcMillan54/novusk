use core::fmt::Write;
use novuskinc::kinfo::{status::set_status, kinfo};
use crate::BOARD_MODLE;
use stm32f4xx_hal::{pac::Peripherals, prelude::*, serial::{config::Config, Serial}};
use stm32f4xx_hal::pac::USART2;
use stm32f4xx_hal::serial::Tx;

pub fn get_serial() -> Tx<USART2> {
    let peripherals = unsafe { Peripherals::steal() };
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
        printk!("Couldn't get serial for STM32f4{}", BOARD_MODLE);
    }

    return tx_uart.unwrap();
}

pub(crate) fn io_init() {
    #[cfg(not(feature = "qemu"))]
    get_serial();
}
