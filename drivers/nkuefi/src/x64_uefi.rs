use core::fmt::{Arguments, Result, Write};
use uart_16550::SerialPort;

pub static mut SERIAL_PORT: SerialPort = unsafe { SerialPort::new(0x3F8) };

pub unsafe fn x86_64_uefi_init() {
    SERIAL_PORT.init();
}

pub struct SerialWriter;

impl SerialWriter {
    pub fn new() -> Self {
        return SerialWriter;
    }

    pub fn write_string(&mut self, string: &str) {
        unsafe { SERIAL_PORT.write_str(string); }
    }
}

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_string(s);
        Ok(())
    }
}

pub fn _serial_print(fmt: Arguments) {
    let mut serial_writer = SerialWriter::new();
    serial_writer.write_fmt(fmt);
}
