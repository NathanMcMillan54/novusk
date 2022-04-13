// #[path = "../../../../char/pl011uart.rs"]
// pub mod pl011uart;

pub mod uart;

use core::borrow::BorrowMut;
use core::fmt::Write;
use novuskinc::console::{KernelConsole, KernelConsoleDriver};
use novuskinc::drivers::Driver;
use uart::Rpi3Uart;
use crate::DEVICE_DRIVERS;

pub struct Rpi3 {
    pub uart: Rpi3Uart
}

impl Rpi3 {
    pub fn board() -> Self {
        return Rpi3 {
            uart: Rpi3Uart::new(),
        };
    }
}

pub unsafe fn rpi3_init() {
    let mut board = Rpi3::board();

    DEVICE_DRIVERS.add_driver(&Rpi3Uart { debug: true, console: KernelConsole { name: "RPi3 UART console", width: 128, height: 64, chars_written: 0 } } as &dyn Driver);

    let mut console = DEVICE_DRIVERS.get_driver("Console Driver").unwrap();
    console.write_string("Console driver!\n", 0, 0);
}
