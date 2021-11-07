use super::{ARM32IO, Arm32Io};
use core::fmt::Arguments;
use super::types::TextMethods;

#[no_mangle]
pub extern "C" fn arch_printk(fmt: Arguments) {
    ARM32IO.lock().write_fmt(fmt);
}

#[no_mangle]
pub extern "C" fn _kernel_main_print(fmt: Arguments) {

}

#[macro_export]
macro_rules! arm32_printk {
    ($($arg:tt)*) => {$crate::kernel::io::text::arch_printk(format_args!($($arg)*))};
}

impl Arm32Io {
    pub fn write_fmt(&mut self, fmt: Arguments) {
        if self.text_method == TextMethods::Hio {
            self.hio_write(fmt);
        }
    }

    pub fn hio_write(&mut self, write: Arguments) {
        hprint!("{}", write);
    }
}
