use core::fmt::{Arguments, Write};
use uart_16550::SerialPort;

pub const IO_PORT: u16 = 0xF38;
pub static mut SERIAL_PORT: SerialPort = unsafe { SerialPort::new(IO_PORT) };

pub unsafe fn _efi_write(fmt: Arguments) {
    SERIAL_PORT.write_fmt(fmt);
}
