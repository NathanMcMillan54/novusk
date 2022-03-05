use core::fmt::{Arguments, Result};
use crate::PrintK;
use spin::Mutex;

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

#[macro_export]
macro_rules! set_printk {
    ($print_fun:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn set_kernel_printer() {
            use printk::PrintK;
            extern "C" {
                static mut PRINTK: PrintK;
            }

            PRINTK.set_printer($print_fun);
        }
    };
}
