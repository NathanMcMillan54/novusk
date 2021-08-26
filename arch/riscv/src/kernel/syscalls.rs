use super::board::Board;
use super::uart::Uart;
use crate::kernel::device::Device;

#[no_mangle]
pub unsafe extern "C" fn sys_write(write: u8) {
    let mut uart = Uart::new(Board::UART0);

    uart.write_byte(write);
}

#[no_mangle]
pub unsafe extern "C" fn sys_read() -> u8 {
    let mut uart = Uart::new(Board::UART0);

    return uart.read();
}
