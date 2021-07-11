use core::fmt::Arguments;
use cortex_m_semihosting::hprint;

#[export_name = "arch_printk"]
#[no_mangle]
pub extern "C" fn arm_printk(fmt: Arguments) {
    hprint!("{}{}", fmt, "\n");
}

#[macro_export]
macro_rules! arm32_printk {
    ($($arg:tt)*) => {$crate::kernel::printk::arm_printk(format_args!($($arg)*))};
}
