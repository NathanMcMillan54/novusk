use core::ptr::write_volatile;
use device::Board;
use mailbox::MailBox;
use crate::RaspberryPi;

pub mod registers;
pub mod mb;
pub mod uart;

#[no_mangle]
pub extern "C" fn device_init() -> (Result<(), &'static str>, &'static str) {
    let mut pi = Rpi2::new();
    pi.init();

    return (Ok(()), "RPi 2");
}

extern "C" {
    static mut BOARD: Board;
    fn rpi2_kernel_init();
}

pub struct Rpi2 {
    pub uart: uart::Rpi2Uart,
}

impl Rpi2 {
    pub fn new() -> Self {
        return Rpi2 {
            uart: uart::Rpi2Uart::new(),
        };
    }

    pub fn init(&mut self) {
        #[cfg(feature = "rpi2")]
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
    fn uart_io_init(&mut self) {
        self.uart.write_bytes(b"Using Qemu Output as UART (for now)\n");
    }

    fn mailbox_init(&mut self) {

    }
}
