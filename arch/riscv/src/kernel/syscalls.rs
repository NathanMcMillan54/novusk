use super::board::Board;
use super::uart::Uart;
use crate::kernel::device::Device;

pub fn write(write: u8) {
    let mut uart = Uart::new(Board::UART0);

    uart.write_byte(write);
}

pub fn read(buf: u8) -> u8 {
    let mut uart = Uart::new(Board::UART0);
    let mut ret = 0;

    for i in 0..buf {
        ret = ret + uart.read();
    }

    return ret;
}

define_syscall!(sys_write, write);
