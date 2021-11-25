use super::{ARM32IO, Arm32Io};
use core::fmt::Arguments;
use super::types::SerialMethods;

#[no_mangle]
pub extern "C" fn arch_printk(fmt: Arguments) {
    ARM32IO.lock().write_fmt(fmt);
}

#[no_mangle]
pub extern "C" fn _kernel_main_print(fmt: Arguments) {

}

#[macro_export]
macro_rules! arm32_printk {
    ($($arg:tt)*) => {$crate::kernel::io::serial::arch_printk(format_args!($($arg)*))};
}

impl Arm32Io {
    pub fn write_fmt(&mut self, fmt: Arguments) {
        if self.serial_method == SerialMethods::Hio {
            self.hio_write(fmt);
        }
    }

    pub fn hio_write(&mut self, write: Arguments) {
        #[cfg(feature = "cortex_m")]
        hprint!("{}", write);
    }
}
