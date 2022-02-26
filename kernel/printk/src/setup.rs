use core::fmt::{Arguments, Result};
use crate::PrintK;

impl PrintK {
    pub fn printing_ok(&self, printer: extern "C" fn(Arguments) -> Result) -> bool {
        if (printer)(format_args!("{}", "")).is_ok() {
            return true;
        } else { return false; }
    }

    pub fn set_printer(&mut self, printer: extern "C" fn(Arguments) -> Result) -> i32 {
        if self.printing_ok(printer) {
            self.kernel_printer = printer;
            return 0;
        } else { return 1; }
    }
}
