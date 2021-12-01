use core::ptr::write_volatile;
use mailbox::MailBox;
use crate::RaspberryPi;

pub mod mb;
pub mod uart;

pub struct Rpi2 {
    pub mb: mb::Rpi2Mb,
    pub uart: uart::Rpi2Uart,
}

impl Rpi2 {
    pub fn new() -> Self {
        return Rpi2 {
            mb: mb::Rpi2Mb::new(),
            uart: uart::Rpi2Uart::new(),
        };
    }

    pub fn init(&self) {
        self.uart_io_init();
    }
}

impl RaspberryPi for Rpi2 {
    fn uart_io_init(&self) {
        self.uart.write_bytes(b"Using Qemu Output as UART (for now)\n");
    }

    fn mailbox_init(&self) {
        self.mb.init();
    }
}
