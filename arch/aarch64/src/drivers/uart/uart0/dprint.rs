use core::fmt::{Arguments, Write, Result};
use super::{Uart0, UART0};

pub unsafe fn _dprint(args: Arguments) {
    UART0.lock().write_fmt(args).unwrap();
}

impl Write for Uart0 {
    fn write_str(&mut self, str: &str) -> Result {
        unsafe { self.write_string(str); }
        Ok(())
    }
}
