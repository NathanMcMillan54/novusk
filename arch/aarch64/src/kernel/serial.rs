use super::uart::Uart;

pub fn serial_init() {
    let mut uart = Uart::new();
    uart.init();
}
