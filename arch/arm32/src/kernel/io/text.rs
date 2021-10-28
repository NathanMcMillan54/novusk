use core::fmt::Arguments;
use crate::kernel::io::{ARM32IO, Arm32Io};

#[no_mangle]
pub extern "C" fn early_printk(fmt: Arguments) {
    ARM32IO.lock().write_fmt(fmt);
}

#[macro_export]
macro_rules! arm32_printk {
    ($($arg:tt)*) => {$crate::kernel::io::text::early_printk(format_args!($($arg)*))};
}

impl Arm32Io {
    pub fn write_fmt(&mut self, fmt: Arguments) {
        if self.text_method == "hio" {
            self.hio_write(fmt);
        }
    }

    pub fn hio_write(&mut self, write: Arguments) {
        hprintln!("{}", write);
    }
}
