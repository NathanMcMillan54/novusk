use core::ptr::write_volatile;
use crate::RaspberryPi;

pub mod uart;

pub struct Rpi2 {
    pub uart: uart::Rpi2Uart,
}

impl Rpi2 {
    pub fn new() -> Self {
        return Rpi2 {
            uart: uart::Rpi2Uart::new(),
        };
    }

    pub fn init(&self) {
        self.uart_io_init();
    }
}

impl RaspberryPi for Rpi2 {
    fn uart_io_init(&self) {
        for c in b"Using Qemu output as UART" {
            unsafe { write_volatile(0x3F20_1000 as *mut u8, *c); }
        }
    }
}
