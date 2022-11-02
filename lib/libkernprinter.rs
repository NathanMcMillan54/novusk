use core::fmt::Write;
use dif::{Dif, DifFieldNames};
use novuskinc::drivers::Driver;

extern "C" {
    static mut DIF: Dif;
}

pub struct Printer(pub &'static dyn Driver);

impl Write for Printer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if unsafe { DIF.get(DifFieldNames::PrintingMethod) } == "Serial" {
            for b in s.as_bytes() {
                self.0.write(*b);
            }
        } else {  }

        Ok(())
    }
}