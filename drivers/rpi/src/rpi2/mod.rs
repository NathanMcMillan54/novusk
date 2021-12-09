use core::ptr::write_volatile;
use device::Board;
use mailbox::MailBox;
use crate::RaspberryPi;

pub mod registers;
pub mod mb;
pub mod uart;

extern "C" {
    static mut BOARD: Board;
    fn rpi2_kernel_init();
}

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
        unsafe { BOARD.set(Board {
            name: "RPi 2",
            peripheral_addr: 0x3F00_0000,
            early_printing_method: "Serial",
            main_printing_method: "Fb",
            arch_init: true,
            kernel_init: false,
            board_specific_kernel: Some(rpi2_kernel_init),
        }); }
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
