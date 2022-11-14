use core::fmt::{Arguments, Result, Write};
use novuskinc::drivers::Driver;
use novuskinc::drivers::names::{SERIAL, SIMPLE_UART};

extern "C" {
    pub fn _early_printk(fmt: Arguments) -> Result;
}

pub struct EarlyPrinter(pub &'static str, pub &'static dyn Driver);

impl Write for EarlyPrinter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if self.0 == SERIAL || self.0 == SIMPLE_UART {
            for b in s.as_bytes() {
                self.1.write(*b);
            }
        } else { return Err(Default::default()); }

        Ok(())
    }
}
