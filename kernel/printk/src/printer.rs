use core::fmt::{Arguments, Write, Result};
use crate::PrintK;

pub(crate) extern "C" fn empty_printk(args: Arguments) -> Result {
    Err(Default::default())
}

impl Write for PrintK {
    fn write_str(&mut self, s: &str) -> Result {
        if self.printing_ok(self.kernel_printer) {
            (self.kernel_printer)(format_args!("{}", s))
        } else { Err(Default::default()) }
    }
}

#[macro_export]
macro_rules! printk {
    ($($args:tt)*) => {
        use core::fmt::Write;
        $crate::PRINTK.lock().write_fmt(format_args!($($args)*))
    };
}
